# H265SliceSegmentLayerParserTest.TestSliceContainsShortTermRefPicSet

- Source: `test/h265_slice_parser_unittest.cc:179`
- Macro: `TEST_F`
- API focus: `H265NalUnitParser::ParseNalUnit`
- Intent: Validate `H265NalUnitParser::ParseNalUnit` behavior for `TestSliceContainsShortTermRefPicSet`.

## Inputs
- Inline byte buffer `pps` (7 bytes)
- Inline byte buffer `slice` (54 bytes)
- Inline byte buffer `sps` (40 bytes)
- Inline byte buffer `vps` (24 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- Collection size matches expected value `1`.
- Condition is true: `bitstream_parser_state.vps.find(0) != bitstream_parser_state.vps.end()`.
- Condition is true: `bitstream_parser_state.sps.find(0) != bitstream_parser_state.sps.end()`.
- Condition is true: `bitstream_parser_state.pps.find(0) != bitstream_parser_state.pps.end()`.
- `result` is non-null.
- `result->nal_unit_payload` is non-null.
- `result->nal_unit_payload->slice_segment_layer` is non-null.
- ... plus 2 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize bitstream parser state
act:
  - execute H265NalUnitParser::ParseNalUnit(...)
assert:
  - verify collection size matches expected value `1`.
  - verify condition is true: `bitstream_parser_state.vps.find(0) != bitstream_parser_state.vps.end()`.
  - verify condition is true: `bitstream_parser_state.sps.find(0) != bitstream_parser_state.sps.end()`.
  - verify condition is true: `bitstream_parser_state.pps.find(0) != bitstream_parser_state.pps.end()`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
