import os
import re

types_dir = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/types"
master_index = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/08_Type_Index.md"

issues = []

def check_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Find lines that look like list items but don't start with '- '
    # e.g. '* ', '+ ', '1. '
    lines = content.split('\n')
    for i, line in enumerate(lines):
        if re.match(r'^\s*[*+]\s+', line):
            issues.append((filepath, i+1, line))

for root, dirs, files in os.walk(types_dir):
    for file in files:
        if file.endswith(".md"):
            filepath = os.path.join(root, file)
            check_file(filepath)

check_file(master_index)

for filepath, line_num, line in issues:
    print(f"File: {filepath}:{line_num} - Issue: {line}")
