# H265SeiParserTest.TestUserDataUnregisteredSei

- Source: `test/h265_sei_parser_unittest.cc:50`
- Macro: `TEST_F`
- API focus: `H265SeiMessageParser::ParseSei`
- Intent: Validate `H265SeiMessageParser::ParseSei` behavior for `TestUserDataUnregisteredSei`.

## Inputs
- Inline byte buffer `buffer` (59 bytes)

## High-Level Checks
- `sei_message` is non-null.
- Field `payload_type` equals expected value `h265nal::SeiType::user_data_unregistered`.
- Field `payload_size` equals expected value `56`.
- `user_data_unregistered_sei` is non-null.
- Field `uuid_iso_iec_11578_1` equals expected value `0x2ca2de09b51747db`.
- Field `uuid_iso_iec_11578_2` equals expected value `0xbb55a4fe7fc2fc4e`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SeiMessageParser::ParseSei(...)
assert:
  - verify `sei_message` is non-null.
  - verify field `payload_type` equals expected value `h265nal::SeiType::user_data_unregistered`.
  - verify field `payload_size` equals expected value `56`.
  - verify `user_data_unregistered_sei` is non-null.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
