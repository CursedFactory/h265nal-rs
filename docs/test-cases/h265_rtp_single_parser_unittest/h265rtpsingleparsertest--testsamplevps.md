# H265RtpSingleParserTest.TestSampleVps

- Source: `test/h265_rtp_single_parser_unittest.cc:24`
- Macro: `TEST_F`
- API focus: `H265RtpSingleParser::ParseRtpSingle`
- Intent: Validate `H265RtpSingleParser::ParseRtpSingle` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (24 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- `rtp_single` is non-null.
- Field `forbidden_zero_bit` equals expected value `0`.
- Field `nal_unit_type` equals expected value `NalUnitType::VPS_NUT`.
- Field `nuh_layer_id` equals expected value `0`.
- Field `nuh_temporal_id_plus1` equals expected value `1`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize bitstream parser state
act:
  - execute H265RtpSingleParser::ParseRtpSingle(...)
assert:
  - verify `rtp_single` is non-null.
  - verify field `forbidden_zero_bit` equals expected value `0`.
  - verify field `nal_unit_type` equals expected value `NalUnitType::VPS_NUT`.
  - verify field `nuh_layer_id` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
