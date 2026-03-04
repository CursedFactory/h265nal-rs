//! References:
//! - Markdown: `docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypeunspecified.md`
//! - C++: `test/h265_common_unittest.cc:40`
//! - Port note: Group 02 / Case 08

mod support;

use support::{assert_bool_result, BLA_W_LP, RSV_NVCL47, UNSPEC50, UNSPEC63};

#[test]
fn test_is_nal_unit_type_unspecified() {
    assert_bool_result(
        "IsNalUnitTypeUnspecified(UNSPEC50)",
        h265nal_sys::common_is_nal_unit_type_unspecified(UNSPEC50),
        true,
    );
    assert_bool_result(
        "IsNalUnitTypeUnspecified(UNSPEC63)",
        h265nal_sys::common_is_nal_unit_type_unspecified(UNSPEC63),
        true,
    );
    assert_bool_result(
        "IsNalUnitTypeUnspecified(BLA_W_LP)",
        h265nal_sys::common_is_nal_unit_type_unspecified(BLA_W_LP),
        false,
    );
    assert_bool_result(
        "IsNalUnitTypeUnspecified(RSV_NVCL47)",
        h265nal_sys::common_is_nal_unit_type_unspecified(RSV_NVCL47),
        false,
    );
}
