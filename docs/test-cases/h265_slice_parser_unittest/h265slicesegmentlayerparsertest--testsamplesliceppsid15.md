# H265SliceSegmentLayerParserTest.TestSampleSlicePpsId15

- Source: `test/h265_slice_parser_unittest.cc:128`
- Macro: `TEST_F`
- API focus: `H265SliceSegmentLayerParser::ParseSliceSegmentLayer`
- Intent: Validate `H265SliceSegmentLayerParser::ParseSliceSegmentLayer` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (160 bytes)
- Shared file-scope buffer `pps` (7 bytes)
- Shared file-scope buffer `sps` (40 bytes)
- Shared file-scope buffer `vps` (24 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- `slice_segment_layer` is non-null.
- `slice_segment_header` is non-null.
- Field `nal_unit_type` equals expected value `19`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize bitstream parser state
act:
  - execute H265SliceSegmentLayerParser::ParseSliceSegmentLayer(...)
assert:
  - verify `slice_segment_layer` is non-null.
  - verify `slice_segment_header` is non-null.
  - verify field `nal_unit_type` equals expected value `19`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
