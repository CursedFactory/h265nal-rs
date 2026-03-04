# H265ProfileTierLevelParserTest.Test022018Standard

- Source: `test/h265_profile_tier_level_parser_unittest.cc:69`
- Macro: `TEST_F`
- API focus: `H265ProfileTierLevelParser::ParseProfileTierLevel`
- Intent: Validate `H265ProfileTierLevelParser::ParseProfileTierLevel` behavior for `Test022018Standard`.

## Inputs
- Inline byte buffer `buffer` (16 bytes)

## High-Level Checks
- `ptls` is non-null.
- Field `profile_space` equals expected value `0`.
- Field `tier_flag` equals expected value `0`.
- Field `profile_idc` equals expected value `1`.
- Sequence contents match the expected reference bytes.
- Field `progressive_source_flag` equals expected value `1`.
- Field `interlaced_source_flag` equals expected value `0`.
- ... plus 19 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265ProfileTierLevelParser::ParseProfileTierLevel(...)
assert:
  - verify `ptls` is non-null.
  - verify field `profile_space` equals expected value `0`.
  - verify field `tier_flag` equals expected value `0`.
  - verify field `profile_idc` equals expected value `1`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
