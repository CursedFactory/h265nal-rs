# h265nal-rs

Rust workspace for HEVC/H.265 NAL parsing with:

- `h265nal-sys`: Rust bindings over the upstream C++ parser
- `h265nal-cli`: Rust CLI front-end for parsing and rendering output
- parity and test-port crates for compatibility validation

The original upstream C++ project docs are preserved in `README.og.md`.

## Workspace Layout

- `crates/h265nal-sys`: FFI surface and typed Rust wrappers
- `crates/h265nal-cli`: command-line parsing tool
- `crates/h265nal-parity`: C++ vs Rust CLI parity checks and reports
- `crates/h265nal-test-ports`: ported test coverage from upstream C++ tests
- `docker/`: parity execution containers

## Build

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

Notes:

- `h265nal-sys` builds the C++ parser via CMake from `build.rs`.
- You do not need to pre-build the C++ project manually.

## Test

Primary runner is `nextest`:

```bash
cargo nextest run
```

Per crate:

```bash
cargo nextest run -p h265nal-cli
```

Doc tests:

```bash
cargo test --doc
```

## Release

Release requirements and step-by-step instructions are documented in `RELEASE.md`.

## Rust CLI (`h265nal-cli`)

Run directly from source:

```bash
cargo run -p h265nal-cli -- --infile video/nvenc.265
```

Or with positional input:

```bash
cargo run -p h265nal-cli -- video/nvenc.265
```

Useful options:

- `--output-format c|json|pretty`
- `--no-as-one-line` for multi-line C-style output
- `--add-offset --add-length --add-parsed-length`
- `--add-checksum --add-resolution --add-contents`
- `--outfile <path>` to write output to file

Example JSON output:

```bash
cargo run -p h265nal-cli -- \
  --infile video/nvenc.265 \
  --output-format json \
  --add-offset --add-length --add-parsed-length
```

## Rust Library (`h265nal-sys`)

`h265nal-sys` exposes typed wrappers around the C ABI, including helpers for:

- Annex B utilities (`count_nalus_annexb`, `dump_annexb_to_stdout`)
- NAL helpers and parse entry points (`nal_unit_parse`, header helpers)
- parser state lifecycle (`BitstreamParserState`)
- selected parser surfaces (`pps_parse`, `sps_parse`, `vps_parse`, `vui_parameters_parse`, `sei_parse`, `slice_segment_layer_parse`)

Example:

```rust
use h265nal_sys::{count_nalus_annexb, sps_parse};

fn inspect(data: &[u8], sps_rbsp: &[u8]) {
    let nalu_count = count_nalus_annexb(data).expect("annexb count failed");
    println!("nal units: {nalu_count}");

    let sps = sps_parse(sps_rbsp).expect("sps parse failed");
    println!(
        "{}x{}",
        sps.pic_width_in_luma_samples,
        sps.pic_height_in_luma_samples
    );
}
```

## Parity Checks

Run parity matrix against Dockerized baseline/local tools:

```bash
H265NAL_PARITY_RUN_DOCKER_TESTS=1 cargo nextest run -p h265nal-parity
```

Generate a markdown parity report:

```bash
cargo run -p h265nal-parity -- report --output parity-report.md
```

## Formatting and Linting

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

## License

BSD-3-Clause. See `LICENSE`.
