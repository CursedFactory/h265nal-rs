# H265RtpApParserTest.TestSampleAUD

- Source: `test/h265_rtp_ap_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265RtpApParser::ParseRtpAp`
- Intent: Validate `H265RtpApParser::ParseRtpAp` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (80 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- `rtp_ap` is non-null.
- Field `forbidden_zero_bit` equals expected value `0`.
- Field `nal_unit_type` equals expected value `NalUnitType::AP`.
- Field `nuh_layer_id` equals expected value `0`.
- Field `nuh_temporal_id_plus1` equals expected value `1`.
- Collection size matches expected value `3`.
- Field `nal_unit_type` equals expected value `NalUnitType::VPS_NUT`.
- ... plus 2 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize bitstream parser state
act:
  - execute H265RtpApParser::ParseRtpAp(...)
assert:
  - verify `rtp_ap` is non-null.
  - verify field `forbidden_zero_bit` equals expected value `0`.
  - verify field `nal_unit_type` equals expected value `NalUnitType::AP`.
  - verify field `nuh_layer_id` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
