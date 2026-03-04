# H265NalUnitParserTest.TestSampleNalUnit

- Source: `test/h265_nal_unit_parser_unittest.cc:22`
- Macro: `TEST_F`
- API focus: `H265NalUnitParser::ParseNalUnit`
- Intent: Validate `H265NalUnitParser::ParseNalUnit` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (24 bytes)
- Setup object: parsing options
- Setup object: bitstream parser state

## High-Level Checks
- `nal_unit` is non-null.
- Field `parsed_length` equals expected value `20`.
- Checksum bytes match the expected golden sequence.
- Computed value `"bfa24594"` equals expected `checksum_printable`.
- Field `forbidden_zero_bit` equals expected value `0`.
- Field `nal_unit_type` equals expected value `NalUnitType::VPS_NUT`.
- Field `nuh_layer_id` equals expected value `0`.
- ... plus 44 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize parsing options
  - initialize bitstream parser state
act:
  - execute H265NalUnitParser::ParseNalUnit(...)
assert:
  - verify `nal_unit` is non-null.
  - verify field `parsed_length` equals expected value `20`.
  - verify checksum bytes match the expected golden sequence.
  - verify computed value `"bfa24594"` equals expected `checksum_printable`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
