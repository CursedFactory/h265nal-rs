# H265SpsParserTest.TestSPSBadWidth

- Source: `test/h265_sps_parser_unittest.cc:161`
- Macro: `TEST_F`
- API focus: `H265SpsParser::ParseSps`
- Intent: Validate invalid/malformed input behavior for `H265SpsParser::ParseSps`.

## Inputs
- Inline byte buffer `buffer` (37 bytes)

## High-Level Checks
- Condition is true: `sps == nullptr`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SpsParser::ParseSps(...)
assert:
  - verify condition is true: `sps == nullptr`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
