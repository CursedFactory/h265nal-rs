//! References:
//! - Markdown: `docs/test-cases/h265_pred_weight_table_parser_unittest/h265predweighttableparsertest--testsamplepredweighttable2.md`
//! - C++: `test/h265_pred_weight_table_parser_unittest.cc:40`
//! - Port note: Group 05 / Case 21

#[test]
fn test_sample_pred_weight_table2() {
    let buffer = [0x10u8, 0xc0, 0x60];
    let pred_weight_table =
        h265nal_sys::pred_weight_table_parse(&buffer, 1, 2).expect("ParsePredWeightTable failed");

    assert_eq!(pred_weight_table.luma_log2_weight_denom, 7);
    assert_eq!(pred_weight_table.delta_chroma_log2_weight_denom, -1);
    assert_eq!(pred_weight_table.luma_weight_l0_flag, vec![0, 0, 0]);
    assert_eq!(pred_weight_table.chroma_weight_l0_flag, vec![0, 0, 0]);
}
