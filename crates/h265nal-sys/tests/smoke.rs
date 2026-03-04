use std::fs;
use std::path::PathBuf;

fn fixture_bytes(file_name: &str) -> Vec<u8> {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../media")
        .join(file_name);
    fs::read(fixture_path).expect("failed to read Annex-B fixture")
}

#[test]
fn annexb_count_smoke() {
    let data = fixture_bytes("pps_fdump_crash.202203.265");
    let count = h265nal_sys::count_nalus_annexb(&data).expect("count failed");
    assert!(count > 0);
}

#[test]
fn abi_version_smoke() {
    assert_eq!(h265nal_sys::abi_version(), h265nal_sys::ABI_VERSION);
}

#[test]
fn annexb_dump_smoke() {
    let data = fixture_bytes("nvenc.265");
    h265nal_sys::dump_annexb_to_stdout(&data, h265nal_sys::DUMP_FLAG_ONE_LINE)
        .expect("dump failed");
}
