//! References:
//! - Markdown: `docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypenonvcl.md`
//! - C++: `test/h265_common_unittest.cc:34`
//! - Port note: Group 02 / Case 07

mod support;

use support::{assert_bool_result, BLA_W_LP, RSV_NVCL43, VPS_NUT};

#[test]
fn test_is_nal_unit_type_non_vcl() {
    assert_bool_result(
        "IsNalUnitTypeNonVcl(VPS_NUT)",
        h265nal_sys::common_is_nal_unit_type_non_vcl(VPS_NUT),
        true,
    );
    assert_bool_result(
        "IsNalUnitTypeNonVcl(RSV_NVCL43)",
        h265nal_sys::common_is_nal_unit_type_non_vcl(RSV_NVCL43),
        true,
    );
    assert_bool_result(
        "IsNalUnitTypeNonVcl(BLA_W_LP)",
        h265nal_sys::common_is_nal_unit_type_non_vcl(BLA_W_LP),
        false,
    );
}
