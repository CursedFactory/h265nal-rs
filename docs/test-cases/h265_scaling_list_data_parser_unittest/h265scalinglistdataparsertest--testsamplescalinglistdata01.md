# H265ScalingListDataParserTest.TestSampleScalingListData01

- Source: `test/h265_scaling_list_data_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265ScalingListDataParser::ParseScalingListData`
- Intent: Validate `H265ScalingListDataParser::ParseScalingListData` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (256 bytes)

## High-Level Checks
- `scaling_list_data` is non-null.
- Collection size matches expected value `4`.
- Sequence contents match the expected reference bytes.
- Collection size matches expected value `6`.
- Collection size matches expected value `0`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265ScalingListDataParser::ParseScalingListData(...)
assert:
  - verify `scaling_list_data` is non-null.
  - verify collection size matches expected value `4`.
  - verify sequence contents match the expected reference bytes.
  - verify collection size matches expected value `6`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
