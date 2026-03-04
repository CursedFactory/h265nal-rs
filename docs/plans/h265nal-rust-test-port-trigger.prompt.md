# H265NAL Rust Test Port Trigger Prompts

## Hop 1: Parent -> squad_developer

Select one group of 5 test cases from GROUP_ID. Analyze the group and prepare for delegation to squad_intern.

Prompt:
```
Analyze GROUP_ID test cases. Select one group of 5 cases for porting. For each case, identify the markdown path (docs/test-cases/...md) and C++ source line (test/...cc:<line>). Prepare delegation prompts for squad_intern.

Output checklist:
- Selected group ID
- List of 5 cases with markdown paths and C++ source lines
- Ready-to-use prompts for squad_intern
```

## Hop 2: squad_developer -> squad_intern

For one selected markdown case, implement Rust test port.

Prompt:
```
Port the test case from CASE_MARKDOWN_PATH (markdown pseudo-case) and C++ source at CPP_SOURCE_LINE to Rust.

Requirements:
- Implement a Rust test port using existing crate API
- Keep 1:1 intent with C++ test and markdown pseudo-case
- If API/feature missing, short-circuit test with TODO comment and record missing feature report entry in docs/plans/h265nal-rust-test-port-missing-features.md
- Include references to both CASE_MARKDOWN_PATH and CPP_SOURCE_LINE in the test code or test module docs

Output checklist:
- Rust test code
- Any missing feature entries added to the report file
```

Replace GROUP_ID, CASE_MARKDOWN_PATH, CPP_SOURCE_LINE with actual values.