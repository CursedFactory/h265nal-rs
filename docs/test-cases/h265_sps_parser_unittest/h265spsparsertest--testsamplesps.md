# H265SpsParserTest.TestSampleSPS

- Source: `test/h265_sps_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265SpsParser::ParseSps`
- Intent: Validate `H265SpsParser::ParseSps` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (37 bytes)

## High-Level Checks
- `sps` is non-null.
- Field `sps_video_parameter_set_id` equals expected value `0`.
- Field `sps_max_sub_layers_minus1` equals expected value `0`.
- Field `sps_temporal_id_nesting_flag` equals expected value `1`.
- Field `profile_space` equals expected value `0`.
- Field `tier_flag` equals expected value `0`.
- Field `profile_idc` equals expected value `1`.
- ... plus 85 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SpsParser::ParseSps(...)
assert:
  - verify `sps` is non-null.
  - verify field `sps_video_parameter_set_id` equals expected value `0`.
  - verify field `sps_max_sub_layers_minus1` equals expected value `0`.
  - verify field `sps_temporal_id_nesting_flag` equals expected value `1`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
