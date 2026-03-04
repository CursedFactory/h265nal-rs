# H265VpsParserTest.TestSampleVPS

- Source: `test/h265_vps_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265VpsParser::ParseVps`
- Intent: Validate `H265VpsParser::ParseVps` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (22 bytes)

## High-Level Checks
- `vps` is non-null.
- Field `vps_video_parameter_set_id` equals expected value `0`.
- Field `vps_base_layer_internal_flag` equals expected value `1`.
- Field `vps_base_layer_available_flag` equals expected value `1`.
- Field `vps_max_layers_minus1` equals expected value `0`.
- Field `vps_max_sub_layers_minus1` equals expected value `0`.
- Field `vps_temporal_id_nesting_flag` equals expected value `1`.
- ... plus 37 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265VpsParser::ParseVps(...)
assert:
  - verify `vps` is non-null.
  - verify field `vps_video_parameter_set_id` equals expected value `0`.
  - verify field `vps_base_layer_internal_flag` equals expected value `1`.
  - verify field `vps_base_layer_available_flag` equals expected value `1`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
