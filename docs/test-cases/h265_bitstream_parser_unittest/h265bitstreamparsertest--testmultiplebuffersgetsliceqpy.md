# H265BitstreamParserTest.TestMultipleBuffersGetSliceQpY

- Source: `test/h265_bitstream_parser_unittest.cc:413`
- Macro: `TEST_F`
- API focus: `H265Utils::GetSliceQpY`
- Intent: Validate `H265Utils::GetSliceQpY` across multiple sequential inputs while preserving expected state.

## Inputs
- Shared file-scope buffer `buffer0` (208 bytes)
- Shared file-scope buffer `buffer1` (144 bytes)
- Shared file-scope buffer `buffer2` (112 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- Collection size matches expected value `1`.
- Computed value `slice_qp_y_vector[0]` equals expected `35`.
- Computed value `slice_qp_y_vector[0]` equals expected `37`.
- Computed value `slice_qp_y_vector[0]` equals expected `42`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize bitstream parser state
act:
  - execute H265Utils::GetSliceQpY(...)
assert:
  - verify collection size matches expected value `1`.
  - verify computed value `slice_qp_y_vector[0]` equals expected `35`.
  - verify computed value `slice_qp_y_vector[0]` equals expected `37`.
  - verify computed value `slice_qp_y_vector[0]` equals expected `42`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
