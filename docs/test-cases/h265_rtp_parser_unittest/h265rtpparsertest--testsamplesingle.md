# H265RtpParserTest.TestSampleSingle

- Source: `test/h265_rtp_parser_unittest.cc:22`
- Macro: `TEST_F`
- API focus: `H265RtpParser::ParseRtp`
- Intent: Validate `H265RtpParser::ParseRtp` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (39 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- `rtp` is non-null.
- Field `forbidden_zero_bit` equals expected value `0`.
- Field `nal_unit_type` equals expected value `NalUnitType::SPS_NUT`.
- Field `nuh_layer_id` equals expected value `0`.
- Field `nuh_temporal_id_plus1` equals expected value `1`.
- Field `pic_width_in_luma_samples` equals expected value `1280`.
- Field `pic_height_in_luma_samples` equals expected value `736`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize bitstream parser state
act:
  - execute H265RtpParser::ParseRtp(...)
assert:
  - verify `rtp` is non-null.
  - verify field `forbidden_zero_bit` equals expected value `0`.
  - verify field `nal_unit_type` equals expected value `NalUnitType::SPS_NUT`.
  - verify field `nuh_layer_id` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
