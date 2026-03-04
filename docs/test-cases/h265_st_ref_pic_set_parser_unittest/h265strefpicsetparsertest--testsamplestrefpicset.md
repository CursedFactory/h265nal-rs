# H265StRefPicSetParserTest.TestSampleStRefPicSet

- Source: `test/h265_st_ref_pic_set_parser_unittest.cc:22`
- Macro: `TEST_F`
- API focus: `H265StRefPicSetParser::ParseStRefPicSet`
- Intent: Validate `H265StRefPicSetParser::ParseStRefPicSet` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (1 bytes)

## High-Level Checks
- `st_ref_pic_set` is non-null.
- Field `num_negative_pics` equals expected value `1`.
- Field `num_positive_pics` equals expected value `0`.
- Sequence contents match the expected reference bytes.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265StRefPicSetParser::ParseStRefPicSet(...)
assert:
  - verify `st_ref_pic_set` is non-null.
  - verify field `num_negative_pics` equals expected value `1`.
  - verify field `num_positive_pics` equals expected value `0`.
  - verify sequence contents match the expected reference bytes.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
