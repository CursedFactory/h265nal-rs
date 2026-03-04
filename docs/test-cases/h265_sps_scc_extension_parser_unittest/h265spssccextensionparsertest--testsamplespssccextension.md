# H265SpsSccExtensionParserTest.TestSampleSpsSccExtension

- Source: `test/h265_sps_scc_extension_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265SpsSccExtensionParser::ParseSpsSccExtension`
- Intent: Validate `H265SpsSccExtensionParser::ParseSpsSccExtension` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (1 bytes)

## High-Level Checks
- `sps_scc_extension` is non-null.
- Field `sps_curr_pic_ref_enabled_flag` equals expected value `1`.
- Field `palette_mode_enabled_flag` equals expected value `0`.
- Field `palette_max_size` equals expected value `0`.
- Field `delta_palette_max_predictor_size` equals expected value `0`.
- Field `sps_palette_predictor_initializers_present_flag` equals expected value `0`.
- Field `sps_num_palette_predictor_initializers_minus1` equals expected value `0`.
- ... plus 3 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SpsSccExtensionParser::ParseSpsSccExtension(...)
assert:
  - verify `sps_scc_extension` is non-null.
  - verify field `sps_curr_pic_ref_enabled_flag` equals expected value `1`.
  - verify field `palette_mode_enabled_flag` equals expected value `0`.
  - verify field `palette_max_size` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
