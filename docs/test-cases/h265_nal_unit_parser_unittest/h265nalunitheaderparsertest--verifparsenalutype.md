# H265NalUnitHeaderParserTest.VerifParseNaluType

- Source: `test/h265_nal_unit_parser_unittest.cc:143`
- Macro: `TEST_F`
- API focus: `H265NalUnitHeaderParser::GetNalUnitType`
- Intent: Validate `H265NalUnitHeaderParser::GetNalUnitType` behavior for `VerifParseNaluType`.

## Inputs
- Inline byte buffer `buffer` (24 bytes)

## High-Level Checks
- Computed value `naluType` equals expected `NalUnitType::BLA_W_LP`.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265NalUnitHeaderParser::GetNalUnitType(...)
assert:
  - verify computed value `naluType` equals expected `NalUnitType::BLA_W_LP`.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
