# H265CommonTest.TestIsNalUnitTypeNonVcl

- Source: `test/h265_common_unittest.cc:34`
- Macro: `TEST_F`
- API focus: `IsNalUnitTypeNonVcl`
- Intent: Validate `IsNalUnitTypeNonVcl` behavior for `TestIsNalUnitTypeNonVcl`.

## Inputs
- Synthetic values/constants assembled directly in test logic

## High-Level Checks
- Condition is true: `IsNalUnitTypeNonVcl(VPS_NUT)`.
- Condition is true: `IsNalUnitTypeNonVcl(RSV_NVCL43)`.
- Condition is false: `IsNalUnitTypeNonVcl(BLA_W_LP)`.

## Pseudocode
```text
arrange:
act:
  - execute IsNalUnitTypeNonVcl(...)
assert:
  - verify condition is true: `IsNalUnitTypeNonVcl(VPS_NUT)`.
  - verify condition is true: `IsNalUnitTypeNonVcl(RSV_NVCL43)`.
  - verify condition is false: `IsNalUnitTypeNonVcl(BLA_W_LP)`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
