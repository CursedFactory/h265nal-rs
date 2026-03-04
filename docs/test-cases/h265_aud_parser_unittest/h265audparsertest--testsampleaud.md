# H265AudParserTest.TestSampleAUD

- Source: `test/h265_aud_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265AudParser::ParseAud`
- Intent: Validate `H265AudParser::ParseAud` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (1 bytes)

## High-Level Checks
- `aud` is non-null.
- Field `pic_type` equals expected value `7`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265AudParser::ParseAud(...)
assert:
  - verify `aud` is non-null.
  - verify field `pic_type` equals expected value `7`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
