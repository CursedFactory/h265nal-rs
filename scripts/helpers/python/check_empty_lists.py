import os
import re

types_dir = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/types"
master_index = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/08_Type_Index.md"

issues = []

def check_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Find headings that are followed by another heading or EOF
    headings = re.findall(r'^##\s+(.*)$', content, flags=re.MULTILINE)
    for i, heading in enumerate(headings):
        if i < len(headings) - 1:
            next_heading = headings[i+1]
            pattern = f"## {heading}\n+## {next_heading}"
            if re.search(pattern, content):
                issues.append((filepath, heading))
        else:
            pattern = f"## {heading}\n+$"
            if re.search(pattern, content):
                issues.append((filepath, heading))

for root, dirs, files in os.walk(types_dir):
    for file in files:
        if file.endswith(".md") and not file.endswith("00_Index.md"):
            filepath = os.path.join(root, file)
            check_file(filepath)

for filepath, heading in issues:
    print(f"File: {filepath} - Empty heading: {heading}")
