# H265SliceSegmentLayerParserTest.TestSampleSlice

- Source: `test/h265_slice_parser_unittest.cc:26`
- Macro: `TEST_F`
- API focus: `H265SliceSegmentLayerParser::ParseSliceSegmentLayer`
- Intent: Validate `H265SliceSegmentLayerParser::ParseSliceSegmentLayer` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (40 bytes)
- Shared file-scope buffer `pps` (7 bytes)
- Shared file-scope buffer `sps` (40 bytes)
- Shared file-scope buffer `vps` (24 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- `slice_segment_layer` is non-null.
- Field `first_slice_segment_in_pic_flag` equals expected value `1`.
- Field `no_output_of_prior_pics_flag` equals expected value `0`.
- Field `slice_pic_parameter_set_id` equals expected value `0`.
- Field `slice_type` equals expected value `2`.
- Field `slice_sao_luma_flag` equals expected value `1`.
- Field `slice_sao_chroma_flag` equals expected value `1`.
- ... plus 3 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize bitstream parser state
act:
  - execute H265SliceSegmentLayerParser::ParseSliceSegmentLayer(...)
assert:
  - verify `slice_segment_layer` is non-null.
  - verify field `first_slice_segment_in_pic_flag` equals expected value `1`.
  - verify field `no_output_of_prior_pics_flag` equals expected value `0`.
  - verify field `slice_pic_parameter_set_id` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
