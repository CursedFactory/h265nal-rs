# H265PredWeightTableParserTest.TestSamplePredWeightTable1

- Source: `test/h265_pred_weight_table_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265PredWeightTableParser::ParsePredWeightTable`
- Intent: Validate `H265PredWeightTableParser::ParsePredWeightTable` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (2 bytes)

## High-Level Checks
- `pred_weight_table` is non-null.
- Field `luma_log2_weight_denom` equals expected value `7`.
- Field `delta_chroma_log2_weight_denom` equals expected value `-1`.
- Sequence contents match the expected reference bytes.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265PredWeightTableParser::ParsePredWeightTable(...)
assert:
  - verify `pred_weight_table` is non-null.
  - verify field `luma_log2_weight_denom` equals expected value `7`.
  - verify field `delta_chroma_log2_weight_denom` equals expected value `-1`.
  - verify sequence contents match the expected reference bytes.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
