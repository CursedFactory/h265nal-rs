# H265ConfigurationBoxParserTest.TestSampleConfigurationBox

- Source: `test/h265_configuration_box_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265ConfigurationBoxParser::ParseConfigurationBox`
- Intent: Validate `H265ConfigurationBoxParser::ParseConfigurationBox` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (107 bytes)
- Setup object: parsing options
- Setup object: bitstream parser state

## High-Level Checks
- `configuration_box` is non-null.
- Field `configurationVersion` equals expected value `1`.
- Field `general_profile_space` equals expected value `0`.
- Field `general_tier_flag` equals expected value `0`.
- Field `general_profile_idc` equals expected value `1`.
- Sequence contents match the expected reference bytes.
- Field `general_constraint_indicator_flags` equals expected value `0x800000000000`.
- ... plus 150 additional field/value checks in source assertions.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
  - initialize parsing options
  - initialize bitstream parser state
act:
  - execute H265ConfigurationBoxParser::ParseConfigurationBox(...)
assert:
  - verify `configuration_box` is non-null.
  - verify field `configurationVersion` equals expected value `1`.
  - verify field `general_profile_space` equals expected value `0`.
  - verify field `general_tier_flag` equals expected value `0`.
  - verify remaining expected fields and invariants from the source test
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
