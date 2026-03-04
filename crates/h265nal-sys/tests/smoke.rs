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

#[test]
fn sps_parse_smoke() {
    let buffer = [
        0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00, 0x00, 0x03, 0x00,
        0x5d, 0xa0, 0x02, 0x80, 0x80, 0x2e, 0x1f, 0x13, 0x96, 0xbb, 0x93, 0x24, 0xbb, 0x95, 0x82,
        0x83, 0x03, 0x01, 0x76, 0x85, 0x09, 0x40,
    ];
    let sps = h265nal_sys::sps_parse(&buffer).expect("sps parse failed");
    assert_eq!(sps.pic_width_in_luma_samples, 1280);
    assert_eq!(sps.pic_size_in_ctbs_y, 920);
}

#[test]
fn slice_segment_layer_parse_smoke() {
    let buffer = [
        0xaf, 0x09, 0x40, 0xf3, 0xb8, 0xd5, 0x39, 0xba, 0x1f, 0xe4, 0xa6, 0x08, 0x5c, 0x6e, 0xb1,
        0x8f, 0x00, 0x38, 0xf1, 0xa6, 0xfc, 0xf1, 0x40, 0x04, 0x3a, 0x86, 0xcb, 0x90, 0x74, 0xce,
        0xf0, 0x46, 0x61, 0x93, 0x72, 0xd6, 0xfc, 0x35, 0xe3, 0xc5,
    ];
    let mut state = h265nal_sys::BitstreamParserState::new().expect("state create failed");
    state.seed_vps(0).expect("seed_vps failed");
    state
        .seed_sps(0, 1, 1, 0, 0, 0, 0)
        .expect("seed_sps failed");
    state.seed_pps(0).expect("seed_pps failed");

    let slice = h265nal_sys::slice_segment_layer_parse(&buffer, 19, &mut state)
        .expect("slice parse failed");
    assert_eq!(slice.slice_type, 2);
}

#[test]
fn sei_parse_smoke() {
    let buffer = [
        0x05, 0x38, 0x2c, 0xa2, 0xde, 0x09, 0xb5, 0x17, 0x47, 0xdb, 0xbb, 0x55, 0xa4, 0xfe, 0x7f,
        0xc2, 0xfc, 0x4e, 0x78, 0x32, 0x36, 0x35, 0x20, 0x28, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20,
        0x33, 0x31, 0x29, 0x20, 0x2d, 0x20, 0x31, 0x2e, 0x33, 0x2b, 0x32, 0x30, 0x2d, 0x36, 0x65,
        0x36, 0x37, 0x35, 0x36, 0x66, 0x39, 0x34, 0x62, 0x32, 0x37, 0x3a, 0x5b, 0x57, 0x69,
    ];
    let sei = h265nal_sys::sei_parse(&buffer).expect("sei parse failed");
    assert_eq!(sei.payload_type, 5);
    assert_eq!(sei.has_user_data_unregistered, 1);
}

#[test]
fn sps_multilayer_extension_parse_smoke() {
    let buffer = [0xffu8];
    let extension = h265nal_sys::sps_multilayer_extension_parse(&buffer)
        .expect("sps multilayer extension parse failed");
    assert_eq!(extension.inter_view_mv_vert_constraint_flag, 1);
}
