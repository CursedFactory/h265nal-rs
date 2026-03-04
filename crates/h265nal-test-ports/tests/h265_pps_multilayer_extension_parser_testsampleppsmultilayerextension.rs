//! References:
//! - Markdown: `docs/test-cases/h265_pps_multilayer_extension_parser_unittest/h265ppsmultilayerextensionparsertest--testsampleppsmultilayerextension.md`
//! - C++: `test/h265_pps_multilayer_extension_parser_unittest.cc:21`
//! - Port note: Group 04 / Case 16

#[test]
#[ignore = "TODO: missing PPS multilayer extension parser API in h265nal-sys"]
fn test_sample_pps_multilayer_extension() {
    // TODO: Implement when h265nal_sys exposes PPS multilayer extension parser
    // Expected buffer: [0x20]
    // Expected fields:
    // - poc_reset_info_present_flag: 0
    // - pps_infer_scaling_list_flag: 0
    // - num_ref_loc_offsets: 0
    // - colour_mapping_enabled_flag: 0
}
