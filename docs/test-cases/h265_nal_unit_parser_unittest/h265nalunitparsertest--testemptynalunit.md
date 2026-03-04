# H265NalUnitParserTest.TestEmptyNalUnit

- Source: `test/h265_nal_unit_parser_unittest.cc:126`
- Macro: `TEST_F`
- API focus: `H265NalUnitParser::ParseNalUnit`
- Intent: Validate `H265NalUnitParser::ParseNalUnit` behavior for `TestEmptyNalUnit`.

## Inputs
- Inline byte buffer `buffer` (0 bytes)
- Setup object: parsing options
- Setup object: bitstream parser state

## High-Level Checks
- Condition is true: `nal_unit == nullptr`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize parsing options
  - initialize bitstream parser state
act:
  - execute H265NalUnitParser::ParseNalUnit(...)
assert:
  - verify condition is true: `nal_unit == nullptr`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
