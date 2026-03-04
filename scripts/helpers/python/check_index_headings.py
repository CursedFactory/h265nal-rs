import os
import re

types_dir = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/types"
master_index = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/08_Type_Index.md"

issues = []

def check_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    headings = re.findall(r'^##\s+(.*)$', content, flags=re.MULTILINE)
    for heading in headings:
        if not heading:
            issues.append((filepath, "Empty heading"))

for root, dirs, files in os.walk(types_dir):
    for file in files:
        if file.endswith("00_Index.md"):
            filepath = os.path.join(root, file)
            check_file(filepath)

check_file(master_index)

for filepath, issue in issues:
    print(f"File: {filepath} - Issue: {issue}")
