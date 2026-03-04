import os
import glob

test_files = glob.glob("/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/test_files/*.md")
index_file = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/09_Test_File_Index.md"

files_changed = 0

for f in test_files:
    with open(f, 'r') as file:
        content = file.read()
    
    new_content = content.replace("## Test cases in this file", "## Test cases")
    new_content = new_content.replace("## Local dependencies (`#include \"...\"`)", "## Local dependencies")
    
    if new_content != content:
        with open(f, 'w') as file:
            file.write(new_content)
        files_changed += 1

with open(index_file, 'r') as file:
    content = file.read()

new_content = content.replace("->", "→")

if new_content != content:
    with open(index_file, 'w') as file:
        file.write(new_content)
    files_changed += 1

print(f"Files changed: {files_changed}")
