//! References:
//! - Markdown: `docs/test-cases/h265_pps_multilayer_extension_parser_unittest/h265ppsmultilayerextensionparsertest--testsampleppsmultilayerextension.md`
//! - C++: `test/h265_pps_multilayer_extension_parser_unittest.cc:21`
//! - Port note: Group 04 / Case 16

#[test]
fn test_sample_pps_multilayer_extension() {
    let buffer = [0x20u8];
    let pps_multilayer_extension = h265nal_sys::pps_multilayer_extension_parse(&buffer)
        .expect("ParsePpsMultilayerExtension failed");

    assert_eq!(pps_multilayer_extension.poc_reset_info_present_flag, 0);
    assert_eq!(pps_multilayer_extension.pps_infer_scaling_list_flag, 0);
    assert_eq!(pps_multilayer_extension.num_ref_loc_offsets, 0);
    assert_eq!(pps_multilayer_extension.colour_mapping_enabled_flag, 0);
}
