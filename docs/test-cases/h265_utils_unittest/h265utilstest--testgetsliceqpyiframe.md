# H265UtilsTest.TestGetSliceQpYIFrame

- Source: `test/h265_utils_unittest.cc:25`
- Macro: `TEST_F`
- API focus: `H265Utils::GetSliceQpY`
- Intent: Validate `H265Utils::GetSliceQpY` behavior for `TestGetSliceQpYIFrame`.

## Inputs
- Inline byte buffer `buffer` (127 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- Collection size matches expected value `1`.
- Computed value `slice_qp_y_vector[0]` equals expected `35`.

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
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
