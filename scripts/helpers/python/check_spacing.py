import os
import re

types_dir = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/types"
master_index = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/08_Type_Index.md"

issues = []

def check_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    if re.search(r'[ \t]+$', content, flags=re.MULTILINE):
        issues.append((filepath, "Trailing spaces"))
    
    if re.search(r'\n{3,}', content):
        issues.append((filepath, "Multiple blank lines"))
        
    if not content.endswith('\n'):
        issues.append((filepath, "No trailing newline"))
        
    if content.endswith('\n\n'):
        issues.append((filepath, "Multiple trailing newlines"))

for root, dirs, files in os.walk(types_dir):
    for file in files:
        if file.endswith(".md"):
            filepath = os.path.join(root, file)
            check_file(filepath)

check_file(master_index)

for filepath, issue in issues:
    print(f"File: {filepath} - Issue: {issue}")
