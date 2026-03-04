# H265VuiParametersParserTest.TestSampleVuiParameters

- Source: `test/h265_vui_parameters_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265VuiParametersParser::ParseVuiParameters`
- Intent: Validate `H265VuiParametersParser::ParseVuiParameters` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (8 bytes)

## High-Level Checks
- `vui_parameters` is non-null.
- Field `aspect_ratio_info_present_flag` equals expected value `0`.
- Field `overscan_info_present_flag` equals expected value `0`.
- Field `video_signal_type_present_flag` equals expected value `1`.
- Field `video_format` equals expected value `2`.
- Field `video_full_range_flag` equals expected value `1`.
- Field `colour_description_present_flag` equals expected value `1`.
- ... plus 18 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265VuiParametersParser::ParseVuiParameters(...)
assert:
  - verify `vui_parameters` is non-null.
  - verify field `aspect_ratio_info_present_flag` equals expected value `0`.
  - verify field `overscan_info_present_flag` equals expected value `0`.
  - verify field `video_signal_type_present_flag` equals expected value `1`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
