# H265CommonMoreRbspDataTest.Run

- Source: `test/h265_common_unittest.cc:129`
- Macro: `TEST_P`
- API focus: `more_rbsp_data`
- Intent: Validate `more_rbsp_data` across a parameterized input table.

## Inputs
- Synthetic values/constants assembled directly in test logic
- Setup object: bit buffer cursor
- Parameterized table: `kH265CommonMoreRbspDataParameterTestcases` (14 cases)

## High-Level Checks
- Returned value matches the expected result for each parameterized case.
- Bit-buffer byte offset remains unchanged after the helper call.
- Bit-buffer bit offset remains unchanged after the helper call.

## Pseudocode
```text
arrange:
  - initialize bit buffer cursor
  - iterate over each parameterized testcase
act:
  - execute more_rbsp_data(...)
assert:
  - verify returned value matches the expected result for each parameterized case.
  - verify bit-buffer byte offset remains unchanged after the helper call.
  - verify bit-buffer bit offset remains unchanged after the helper call.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
