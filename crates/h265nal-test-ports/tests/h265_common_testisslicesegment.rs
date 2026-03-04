//! References:
//! - Markdown: `docs/test-cases/h265_common_unittest/h265commontest--testisslicesegment.md`
//! - C++: `test/h265_common_unittest.cc:22`
//! - Port note: Group 02 / Case 10

mod support;

use support::{assert_bool_result, TRAIL_N, VPS_NUT};

#[test]
fn test_is_slice_segment() {
    assert_bool_result(
        "IsSliceSegment(TRAIL_N)",
        h265nal_sys::common_is_slice_segment(TRAIL_N),
        true,
    );
    assert_bool_result(
        "IsSliceSegment(VPS_NUT)",
        h265nal_sys::common_is_slice_segment(VPS_NUT),
        false,
    );
}
