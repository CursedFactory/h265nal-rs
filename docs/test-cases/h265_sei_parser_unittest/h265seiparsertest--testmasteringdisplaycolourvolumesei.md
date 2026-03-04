# H265SeiParserTest.TestMasteringDisplayColourVolumeSei

- Source: `test/h265_sei_parser_unittest.cc:96`
- Macro: `TEST_F`
- API focus: `H265SeiMessageParser::ParseSei`
- Intent: Validate `H265SeiMessageParser::ParseSei` behavior for `TestMasteringDisplayColourVolumeSei`.

## Inputs
- Inline byte buffer `buffer` (34 bytes)

## High-Level Checks
- `sei_message` is non-null.
- Field `payload_type` equals expected value `h265nal::SeiType::mastering_display_colour_volume`.
- Field `payload_size` equals expected value `24`.
- `mastering_display_sei` is non-null.
- Field `display_primaries_x[0]` equals expected value `0x8449`.
- Field `display_primaries_y[0]` equals expected value `0x7d00`.
- Field `display_primaries_x[1]` equals expected value `0x335c`.
- ... plus 7 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SeiMessageParser::ParseSei(...)
assert:
  - verify `sei_message` is non-null.
  - verify field `payload_type` equals expected value `h265nal::SeiType::mastering_display_colour_volume`.
  - verify field `payload_size` equals expected value `24`.
  - verify `mastering_display_sei` is non-null.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
