//! References:
//! - Markdown: `docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypevcl.md`
//! - C++: `test/h265_common_unittest.cc:27`
//! - Port note: Group 02 / Case 09

mod support;

use support::{assert_bool_result, BLA_W_LP, RSV_NVCL43, VPS_NUT};

#[test]
fn test_is_nal_unit_type_vcl() {
    assert_bool_result(
        "IsNalUnitTypeVcl(BLA_W_LP)",
        h265nal_sys::common_is_nal_unit_type_vcl(BLA_W_LP),
        true,
    );
    assert_bool_result(
        "IsNalUnitTypeVcl(VPS_NUT)",
        h265nal_sys::common_is_nal_unit_type_vcl(VPS_NUT),
        false,
    );
    assert_bool_result(
        "IsNalUnitTypeVcl(RSV_NVCL43)",
        h265nal_sys::common_is_nal_unit_type_vcl(RSV_NVCL43),
        false,
    );
}
