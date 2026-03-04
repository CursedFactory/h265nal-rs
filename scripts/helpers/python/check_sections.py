import os
import glob

files = glob.glob("/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/test_files/*.md")

expected_sections = [
    "## Purpose",
    "## Test cases in this file",
    "## Linked source modules",
    "## Linked types/classes",
    "## Local dependencies (`#include \"...\"`)",
    "## Related map notes",
    "## Dependency/context notes"
]

for f in files:
    with open(f, 'r') as file:
        content = file.read()
        for section in expected_sections:
            if section not in content:
                print(f"Missing '{section}' in {f}")
