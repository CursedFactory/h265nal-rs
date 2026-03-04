# H265RtpFuParserTest.TestSampleStart

- Source: `test/h265_rtp_fu_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265RtpFuParser::ParseRtpFu`
- Intent: Validate `H265RtpFuParser::ParseRtpFu` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (28 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- `rtp_fu` is non-null.
- Field `forbidden_zero_bit` equals expected value `0`.
- Field `nal_unit_type` equals expected value `NalUnitType::FU`.
- Field `nuh_layer_id` equals expected value `0`.
- Field `nuh_temporal_id_plus1` equals expected value `1`.
- Field `s_bit` equals expected value `1`.
- Field `e_bit` equals expected value `0`.
- ... plus 9 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize bitstream parser state
act:
  - execute H265RtpFuParser::ParseRtpFu(...)
assert:
  - verify `rtp_fu` is non-null.
  - verify field `forbidden_zero_bit` equals expected value `0`.
  - verify field `nal_unit_type` equals expected value `NalUnitType::FU`.
  - verify field `nuh_layer_id` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
