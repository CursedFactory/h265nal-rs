# H265PpsParserTest.TestSamplePPS

- Source: `test/h265_pps_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265PpsParser::ParsePps`
- Intent: Validate `H265PpsParser::ParsePps` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (6 bytes)

## High-Level Checks
- `pps` is non-null.
- Field `pps_pic_parameter_set_id` equals expected value `0`.
- Field `pps_seq_parameter_set_id` equals expected value `0`.
- Field `dependent_slice_segments_enabled_flag` equals expected value `0`.
- Field `output_flag_present_flag` equals expected value `0`.
- Field `num_extra_slice_header_bits` equals expected value `0`.
- Field `sign_data_hiding_enabled_flag` equals expected value `0`.
- ... plus 24 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265PpsParser::ParsePps(...)
assert:
  - verify `pps` is non-null.
  - verify field `pps_pic_parameter_set_id` equals expected value `0`.
  - verify field `pps_seq_parameter_set_id` equals expected value `0`.
  - verify field `dependent_slice_segments_enabled_flag` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
