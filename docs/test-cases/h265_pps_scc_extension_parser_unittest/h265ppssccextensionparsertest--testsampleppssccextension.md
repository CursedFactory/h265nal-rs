# H265PpsSccExtensionParserTest.TestSamplePpsSccExtension

- Source: `test/h265_pps_scc_extension_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265PpsSccExtensionParser::ParsePpsSccExtension`
- Intent: Validate `H265PpsSccExtensionParser::ParsePpsSccExtension` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (1 bytes)

## High-Level Checks
- `pps_scc_extension` is non-null.
- Field `pps_curr_pic_ref_enabled_flag` equals expected value `0`.
- Field `residual_adaptive_colour_transform_enabled_flag` equals expected value `0`.
- Field `pps_palette_predictor_initializer_present_flag` equals expected value `0`.
- Collection size matches expected value `0`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265PpsSccExtensionParser::ParsePpsSccExtension(...)
assert:
  - verify `pps_scc_extension` is non-null.
  - verify field `pps_curr_pic_ref_enabled_flag` equals expected value `0`.
  - verify field `residual_adaptive_colour_transform_enabled_flag` equals expected value `0`.
  - verify field `pps_palette_predictor_initializer_present_flag` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
