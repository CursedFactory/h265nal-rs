//! References:
//! - Markdown: `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitparsertest--testsamplenalunit.md`
//! - C++: `test/h265_nal_unit_parser_unittest.cc:22`
//! - Port note: Group 03 / Case 15

#[test]
#[ignore = "TODO: nal_unit_parse exists, but checksum/VPS payload parity fields are still missing"]
fn test_sample_nal_unit() {
    // TODO: extend h265nal_sys::nal_unit_parse surface with checksum bytes and
    // VPS payload/profile_tier_level fields to mirror C++ checks 1:1.
    assert!(true);
}
