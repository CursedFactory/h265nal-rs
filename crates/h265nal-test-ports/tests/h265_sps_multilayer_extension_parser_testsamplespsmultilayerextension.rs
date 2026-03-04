//! References:
//! - Markdown: `docs/test-cases/h265_sps_multilayer_extension_parser_unittest/h265spsmultilayerextensionparsertest--testsamplespsmultilayerextension.md`
//! - C++: `test/h265_sps_multilayer_extension_parser_unittest.cc:21`
//! - Port note: Group 09 / Case 43

#[test]
fn test_sample_sps_multilayer_extension() {
    let buffer = [0xffu8];
    let extension = h265nal_sys::sps_multilayer_extension_parse(&buffer)
        .expect("ParseSpsMultilayerExtension failed");
    assert_eq!(extension.inter_view_mv_vert_constraint_flag, 1);
}
