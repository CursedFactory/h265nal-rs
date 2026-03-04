# H265SeiParserTest.TestAlphaChannelInfoSei

- Source: `test/h265_sei_parser_unittest.cc:76`
- Macro: `TEST_F`
- API focus: `H265SeiMessageParser::ParseSei`
- Intent: Validate `H265SeiMessageParser::ParseSei` behavior for `TestAlphaChannelInfoSei`.

## Inputs
- Inline byte buffer `buffer` (7 bytes)

## High-Level Checks
- `sei_message` is non-null.
- `alpha_channel_info_sei` is non-null.
- Field `alpha_channel_cancel_flag` equals expected value `0`.
- Field `alpha_channel_use_idc` equals expected value `0`.
- Field `alpha_channel_bit_depth_minus8` equals expected value `0`.
- Field `alpha_transparent_value` equals expected value `0`.
- Field `alpha_opaque_value` equals expected value `255`.
- ... plus 2 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SeiMessageParser::ParseSei(...)
assert:
  - verify `sei_message` is non-null.
  - verify `alpha_channel_info_sei` is non-null.
  - verify field `alpha_channel_cancel_flag` equals expected value `0`.
  - verify field `alpha_channel_use_idc` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
