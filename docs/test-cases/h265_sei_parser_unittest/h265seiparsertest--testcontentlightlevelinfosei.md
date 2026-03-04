# H265SeiParserTest.TestContentLightLevelInfoSei

- Source: `test/h265_sei_parser_unittest.cc:143`
- Macro: `TEST_F`
- API focus: `H265SeiMessageParser::ParseSei`
- Intent: Validate `H265SeiMessageParser::ParseSei` behavior for `TestContentLightLevelInfoSei`.

## Inputs
- Inline byte buffer `buffer` (6 bytes)

## High-Level Checks
- `sei_message` is non-null.
- Field `payload_type` equals expected value `h265nal::SeiType::content_light_level_info`.
- Field `payload_size` equals expected value `4`.
- `content_light_sei` is non-null.
- Field `max_content_light_level` equals expected value `1000`.
- Field `max_pic_average_light_level` equals expected value `400`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SeiMessageParser::ParseSei(...)
assert:
  - verify `sei_message` is non-null.
  - verify field `payload_type` equals expected value `h265nal::SeiType::content_light_level_info`.
  - verify field `payload_size` equals expected value `4`.
  - verify `content_light_sei` is non-null.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
