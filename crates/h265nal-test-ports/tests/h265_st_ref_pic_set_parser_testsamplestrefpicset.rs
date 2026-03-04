//! References:
//! - Markdown: `docs/test-cases/h265_st_ref_pic_set_parser_unittest/h265strefpicsetparsertest--testsamplestrefpicset.md`
//! - C++: `test/h265_st_ref_pic_set_parser_unittest.cc:22`
//! - Port note: Group 10 / Case 50

#[test]
fn test_sample_st_ref_pic_set() {
    let buffer = [0x5d];
    let st_ref_pic_set =
        h265nal_sys::st_ref_pic_set_parse(&buffer, 0, 1, 1).expect("ParseStRefPicSet failed");

    assert_eq!(st_ref_pic_set.num_negative_pics, 1);
    assert_eq!(st_ref_pic_set.num_positive_pics, 0);
    assert_eq!(st_ref_pic_set.delta_poc_s0_minus1_size, 1);
    assert_eq!(st_ref_pic_set.delta_poc_s0_minus1[0], 0);
}
