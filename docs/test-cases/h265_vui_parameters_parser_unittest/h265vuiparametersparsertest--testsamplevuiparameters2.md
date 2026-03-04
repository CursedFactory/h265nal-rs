# H265VuiParametersParserTest.TestSampleVuiParameters2

- Source: `test/h265_vui_parameters_parser_unittest.cc:61`
- Macro: `TEST_F`
- API focus: `H265VuiParametersParser::ParseVuiParameters`
- Intent: Validate `H265VuiParametersParser::ParseVuiParameters` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (16 bytes)

## High-Level Checks
- `vui_parameters` is non-null.
- Field `aspect_ratio_info_present_flag` equals expected value `1`.
- Field `aspect_ratio_idc` equals expected value `255`.
- Field `sar_width` equals expected value `128`.
- Field `sar_height` equals expected value `117`.
- Field `overscan_info_present_flag` equals expected value `0`.
- Field `video_signal_type_present_flag` equals expected value `1`.
- ... plus 14 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265VuiParametersParser::ParseVuiParameters(...)
assert:
  - verify `vui_parameters` is non-null.
  - verify field `aspect_ratio_info_present_flag` equals expected value `1`.
  - verify field `aspect_ratio_idc` equals expected value `255`.
  - verify field `sar_width` equals expected value `128`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
