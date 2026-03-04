# H265PpsMultilayerExtensionParserTest.TestSamplePpsMultilayerExtension

- Source: `test/h265_pps_multilayer_extension_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265PpsMultilayerExtensionParser::ParsePpsMultilayerExtension`
- Intent: Validate `H265PpsMultilayerExtensionParser::ParsePpsMultilayerExtension` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (1 bytes)

## High-Level Checks
- `pps_multilayer_extension` is non-null.
- Field `poc_reset_info_present_flag` equals expected value `0`.
- Field `pps_infer_scaling_list_flag` equals expected value `0`.
- Field `num_ref_loc_offsets` equals expected value `0`.
- Field `colour_mapping_enabled_flag` equals expected value `0`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265PpsMultilayerExtensionParser::ParsePpsMultilayerExtension(...)
assert:
  - verify `pps_multilayer_extension` is non-null.
  - verify field `poc_reset_info_present_flag` equals expected value `0`.
  - verify field `pps_infer_scaling_list_flag` equals expected value `0`.
  - verify field `num_ref_loc_offsets` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
