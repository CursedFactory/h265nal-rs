# H265SpsMultilayerExtensionParserTest.TestSampleSpsMultilayerExtension

- Source: `test/h265_sps_multilayer_extension_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265SpsMultilayerExtensionParser::ParseSpsMultilayerExtension`
- Intent: Validate `H265SpsMultilayerExtensionParser::ParseSpsMultilayerExtension` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (1 bytes)

## High-Level Checks
- `sps_multilayer_extension` is non-null.
- Field `inter_view_mv_vert_constraint_flag` equals expected value `1`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SpsMultilayerExtensionParser::ParseSpsMultilayerExtension(...)
assert:
  - verify `sps_multilayer_extension` is non-null.
  - verify field `inter_view_mv_vert_constraint_flag` equals expected value `1`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
