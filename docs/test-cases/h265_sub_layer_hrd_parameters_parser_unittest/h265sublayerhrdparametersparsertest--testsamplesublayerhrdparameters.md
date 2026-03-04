# H265SubLayerHrdParametersParserTest.TestSampleSubLayerHrdParameters

- Source: `test/h265_sub_layer_hrd_parameters_parser_unittest.cc:21`
- Macro: `TEST_F`
- API focus: `H265SubLayerHrdParametersParser::ParseSubLayerHrdParameters`
- Intent: Validate `H265SubLayerHrdParametersParser::ParseSubLayerHrdParameters` on a representative sample input.

## Inputs
- Inline byte buffer `buffer` (9 bytes)

## High-Level Checks
- `sub_layer_hrd_parameters` is non-null.
- Sequence contents match the expected reference bytes.

## Pseudocode
```text
arrange:
  - prepare input bytes for the scenario
act:
  - execute H265SubLayerHrdParametersParser::ParseSubLayerHrdParameters(...)
assert:
  - verify `sub_layer_hrd_parameters` is non-null.
  - verify sequence contents match the expected reference bytes.
```

## Porting Notes
- Preserve state setup order before calling parse helpers.
- Keep checks semantic (field/value intent), then add exact-value asserts in target language.
