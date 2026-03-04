use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    configure_cmake_binary();

    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set by Cargo"),
    );
    let source_root = manifest_dir
        .join("native")
        .canonicalize()
        .expect("failed to resolve bundled source root");

    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}",
        source_root.join("CMakeLists.txt").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        source_root.join("src").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        source_root.join("include").display()
    );

    let mut config = cmake::Config::new(&source_root);
    config
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_CLANG_FUZZER", "OFF")
        .define("CMAKE_CXX_CLANG_TIDY", "")
        .build_target("h265nal");

    let dst = config.build();
    let lib_dir = resolve_library_directory(&dst).unwrap_or_else(|| {
        panic!(
            "could not locate built h265nal library under {}",
            dst.display()
        )
    });

    println!("cargo:warning=h265nal cmake root: {}", dst.display());
    println!("cargo:warning=h265nal link dir: {}", lib_dir.display());

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=h265nal");

    emit_cpp_runtime_link();
}

fn configure_cmake_binary() {
    if env::var_os("CMAKE").is_some() {
        return;
    }

    if Command::new("cmake").arg("--version").output().is_ok() {
        // Prefer cmake discovered on PATH, which works with proto/prototools and CI.
        println!("cargo:warning=using cmake from PATH");
        return;
    }

    panic!(
        "cmake was not found. Install it via your toolchain manager (see .prototools) \
         or set CMAKE=/path/to/cmake"
    );
}

fn resolve_library_directory(dst: &Path) -> Option<PathBuf> {
    let mut candidates = vec![
        dst.join("build/src"),
        dst.join("build"),
        dst.join("lib"),
        dst.join("lib64"),
    ];

    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if target_env == "msvc" {
        candidates.extend([
            dst.join("build/src/Release"),
            dst.join("build/Release"),
            dst.join("Release"),
        ]);
    }

    let library_names = [
        "libh265nal.a",
        "h265nal.lib",
        "libh265nal.lib",
        "libh265nal.dylib",
        "libh265nal.so",
        "h265nal.dll",
    ];

    candidates
        .into_iter()
        .find(|dir| dir.is_dir() && library_names.iter().any(|name| dir.join(name).exists()))
}

fn emit_cpp_runtime_link() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    match target_os.as_str() {
        "macos" | "ios" | "tvos" | "watchos" => {
            println!("cargo:rustc-link-lib=dylib=c++");
        }
        "linux" | "android" | "freebsd" | "openbsd" | "netbsd" | "dragonfly" => {
            if target_env != "msvc" {
                println!("cargo:rustc-link-lib=dylib=stdc++");
            }
        }
        _ => {}
    }
}
