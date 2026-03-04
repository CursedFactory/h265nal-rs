# H265UtilsTest.TestGetSliceQpYIFrameBroken

- Source: `test/h265_utils_unittest.cc:74`
- Macro: `TEST_F`
- API focus: `H265Utils::GetSliceQpY`
- Intent: Validate invalid/malformed input behavior for `H265Utils::GetSliceQpY`.

## Inputs
- Inline byte buffer `buffer` (7 bytes)
- Setup object: bitstream parser state

## High-Level Checks
- Collection size matches expected value `0`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize bitstream parser state
act:
  - execute H265Utils::GetSliceQpY(...)
assert:
  - verify collection size matches expected value `0`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
