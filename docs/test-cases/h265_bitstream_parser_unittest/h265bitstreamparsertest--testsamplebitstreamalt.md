# H265BitstreamParserTest.TestSampleBitstreamAlt

- Source: `test/h265_bitstream_parser_unittest.cc:135`
- Macro: `TEST_F`
- API focus: `H265BitstreamParser::ParseBitstream`
- Intent: Validate `H265BitstreamParser::ParseBitstream` on a representative sample input.

## Inputs
- Shared file-scope buffer `buffer` (176 bytes)
- Setup object: parsing options

## High-Level Checks
- `bitstream` is non-null.
- Collection size matches expected value `4`.
- Field `offset` equals expected value `counter`.
- Field `length` equals expected value `length`.
- Field `parsed_length` equals expected value `20`.
- Checksum bytes match the expected golden sequence.
- Field `parsed_length` equals expected value `36`.
- ... plus 2 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize parsing options
act:
  - execute H265BitstreamParser::ParseBitstream(...)
assert:
  - verify `bitstream` is non-null.
  - verify collection size matches expected value `4`.
  - verify field `offset` equals expected value `counter`.
  - verify field `length` equals expected value `length`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
