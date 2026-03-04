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

#[test]
fn pps_parse_smoke() {
    let buffer = [0xc0u8, 0xf3, 0xc0, 0x02, 0x10, 0x00];
    let pps = h265nal_sys::pps_parse(&buffer).expect("pps parse failed");
    assert_eq!(pps.pps_pic_parameter_set_id, 0);
    assert_eq!(pps.cabac_init_present_flag, 1);
}

#[test]
fn profile_tier_level_parse_smoke() {
    let buffer = [
        0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00, 0x00, 0x03, 0x00, 0x5d,
        0xac, 0x59,
    ];
    let ptl = h265nal_sys::profile_tier_level_parse(&buffer, true, 0)
        .expect("profile_tier_level parse failed");
    assert_eq!(ptl.general.profile_idc, 1);
    assert_eq!(ptl.general_level_idc, 93);
}

#[test]
fn nal_unit_helpers_smoke() {
    let buffer = [
        0x40, 0x01, 0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x03, 0x00, 0x5d, 0xac, 0x59, 0x00,
    ];
    let mut state = h265nal_sys::BitstreamParserState::new().expect("state create failed");
    let nal_unit =
        h265nal_sys::nal_unit_parse(&buffer, &mut state, false).expect("nal parse failed");
    assert_eq!(nal_unit.nal_unit_type, 32);

    let header_only_type =
        h265nal_sys::nal_unit_header_get_nal_unit_type(&buffer).expect("type parse failed");
    assert_eq!(header_only_type, 32);
}
