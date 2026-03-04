# H265BitstreamParserTest.TestSampleBitstream

- Source: `test/h265_bitstream_parser_unittest.cc:57`
- Macro: `TEST_F`
- API focus: `H265BitstreamParser::ParseBitstream`
- Intent: Validate `H265BitstreamParser::ParseBitstream` on a representative sample input.

## Inputs
- Shared file-scope buffer `buffer` (176 bytes)
- Setup object: parsing options
- Setup object: bitstream parser state

## High-Level Checks
- `bitstream` is non-null.
- Collection size matches expected value `4`.
- Field `forbidden_zero_bit` equals expected value `0`.
- Field `nal_unit_type` equals expected value `NalUnitType::VPS_NUT`.
- Field `nuh_layer_id` equals expected value `0`.
- Field `nuh_temporal_id_plus1` equals expected value `1`.
- Checksum bytes match the expected golden sequence.
- ... plus 3 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize parsing options
  - initialize bitstream parser state
act:
  - execute H265BitstreamParser::ParseBitstream(...)
assert:
  - verify `bitstream` is non-null.
  - verify collection size matches expected value `4`.
  - verify field `forbidden_zero_bit` equals expected value `0`.
  - verify field `nal_unit_type` equals expected value `NalUnitType::VPS_NUT`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
