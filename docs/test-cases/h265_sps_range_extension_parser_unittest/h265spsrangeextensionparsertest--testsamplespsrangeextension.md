# H265SpsRangeExtensionParserTest.TestSampleSpsRangeExtension

- Source: `test/h265_sps_range_extension_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265SpsRangeExtensionParser::ParseSpsRangeExtension`
- Intent: Validate `H265SpsRangeExtensionParser::ParseSpsRangeExtension` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (2 bytes)

## High-Level Checks
- `sps_range_extension` is non-null.
- Field `transform_skip_rotation_enabled_flag` equals expected value `1`.
- Field `transform_skip_context_enabled_flag` equals expected value `0`.
- Field `implicit_rdpcm_enabled_flag` equals expected value `1`.
- Field `explicit_rdpcm_enabled_flag` equals expected value `0`.
- Field `extended_precision_processing_flag` equals expected value `1`.
- Field `intra_smoothing_disabled_flag` equals expected value `0`.
- ... plus 3 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SpsRangeExtensionParser::ParseSpsRangeExtension(...)
assert:
  - verify `sps_range_extension` is non-null.
  - verify field `transform_skip_rotation_enabled_flag` equals expected value `1`.
  - verify field `transform_skip_context_enabled_flag` equals expected value `0`.
  - verify field `implicit_rdpcm_enabled_flag` equals expected value `1`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
