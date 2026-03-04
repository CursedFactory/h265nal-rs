# H265HrdParametersParserTest.TestSampleHrdParameters

- Source: `test/h265_hrd_parameters_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265HrdParametersParser::ParseHrdParameters`
- Intent: Validate `H265HrdParametersParser::ParseHrdParameters` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (14 bytes)

## High-Level Checks
- `hrd_parameters` is non-null.
- Field `nal_hrd_parameters_present_flag` equals expected value `1`.
- Field `vcl_hrd_parameters_present_flag` equals expected value `0`.
- Field `sub_pic_hrd_params_present_flag` equals expected value `0`.
- Field `tick_divisor_minus2` equals expected value `0`.
- Field `du_cpb_removal_delay_increment_length_minus1` equals expected value `0`.
- Field `sub_pic_cpb_params_in_pic_timing_sei_flag` equals expected value `0`.
- ... plus 10 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265HrdParametersParser::ParseHrdParameters(...)
assert:
  - verify `hrd_parameters` is non-null.
  - verify field `nal_hrd_parameters_present_flag` equals expected value `1`.
  - verify field `vcl_hrd_parameters_present_flag` equals expected value `0`.
  - verify field `sub_pic_hrd_params_present_flag` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
