# H265Sps3dExtensionParserTest.TestSampleSps3dExtension

- Source: `test/h265_sps_3d_extension_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265Sps3dExtensionParser::ParseSps3dExtension`
- Intent: Validate `H265Sps3dExtensionParser::ParseSps3dExtension` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (2 bytes)

## High-Level Checks
- `sps_3d_extension` is non-null.
- Sequence contents match the expected reference bytes.
- Field `log2_ivmc_sub_pb_size_minus3` equals expected value `0`.
- Field `iv_res_pred_enabled_flag` equals expected value `1`.
- Field `depth_ref_enabled_flag` equals expected value `1`.
- Field `vsp_mc_enabled_flag` equals expected value `1`.
- Field `dbbp_enabled_flag` equals expected value `1`.
- ... plus 7 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265Sps3dExtensionParser::ParseSps3dExtension(...)
assert:
  - verify `sps_3d_extension` is non-null.
  - verify sequence contents match the expected reference bytes.
  - verify field `log2_ivmc_sub_pb_size_minus3` equals expected value `0`.
  - verify field `iv_res_pred_enabled_flag` equals expected value `1`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
