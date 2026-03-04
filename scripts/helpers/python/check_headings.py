import os
import re

types_dir = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/types"
required_headings = [
    "## Kind",
    "## Defined in",
    "## Purpose in this repo",
    "## Key API touchpoints",
    "## Direct type dependencies",
    "## Used by source files",
    "## High-level test coverage"
]

issues = []

for root, dirs, files in os.walk(types_dir):
    for file in files:
        if file.endswith(".md") and not file.endswith("00_Index.md"):
            filepath = os.path.join(root, file)
            with open(filepath, 'r') as f:
                content = f.read()
            
            headings = re.findall(r'^##\s+(.*)$', content, flags=re.MULTILINE)
            headings_with_hash = ["## " + h for h in headings]
            
            missing = [h for h in required_headings if h not in headings_with_hash]
            extra = [h for h in headings_with_hash if h not in required_headings]
            
            if missing or extra:
                issues.append((filepath, missing, extra))

for filepath, missing, extra in issues:
    print(f"File: {filepath}")
    if missing:
        print(f"  Missing: {missing}")
    if extra:
        print(f"  Extra: {extra}")
