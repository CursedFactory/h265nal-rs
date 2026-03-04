# H265SeiParserTest.TestUserDataRegisteredItuTT35Sei

- Source: `test/h265_sei_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265SeiMessageParser::ParseSei`
- Intent: Validate `H265SeiMessageParser::ParseSei` behavior for `TestUserDataRegisteredItuTT35Sei`.

## Inputs
- Inline byte buffer `buffer` (73 bytes)

## High-Level Checks
- `sei_message` is non-null.
- Field `payload_type` equals expected value `h265nal::SeiType::user_data_registered_itu_t_t35`.
- Field `payload_size` equals expected value `71`.
- `user_data_registered_itu_t_t35_sei` is non-null.
- Field `itu_t_t35_country_code` equals expected value `181`.
- Field `itu_t_t35_country_code_extension_byte` equals expected value `0`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SeiMessageParser::ParseSei(...)
assert:
  - verify `sei_message` is non-null.
  - verify field `payload_type` equals expected value `h265nal::SeiType::user_data_registered_itu_t_t35`.
  - verify field `payload_size` equals expected value `71`.
  - verify `user_data_registered_itu_t_t35_sei` is non-null.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
