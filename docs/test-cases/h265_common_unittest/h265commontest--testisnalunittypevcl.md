# H265CommonTest.TestIsNalUnitTypeVcl

- Source: `test/h265_common_unittest.cc:27`
- Macro: `TEST_F`
- API focus: `IsNalUnitTypeVcl`
- Intent: Validate `IsNalUnitTypeVcl` behavior for `TestIsNalUnitTypeVcl`.

## Inputs
- Synthetic values/constants assembled directly in test logic

## High-Level Checks
- Condition is true: `IsNalUnitTypeVcl(BLA_W_LP)`.
- Condition is false: `IsNalUnitTypeVcl(VPS_NUT)`.
- Condition is false: `IsNalUnitTypeVcl(RSV_NVCL43)`.

## Pseudocode
```text
arrange:
act:
  - execute IsNalUnitTypeVcl(...)
assert:
  - verify condition is true: `IsNalUnitTypeVcl(BLA_W_LP)`.
  - verify condition is false: `IsNalUnitTypeVcl(VPS_NUT)`.
  - verify condition is false: `IsNalUnitTypeVcl(RSV_NVCL43)`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
