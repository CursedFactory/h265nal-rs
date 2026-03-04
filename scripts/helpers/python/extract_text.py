import glob
import re

files = glob.glob("/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/test_files/*.md")
files.append("/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/09_Test_File_Index.md")

all_text = set()

for f in files:
    with open(f, 'r') as file:
        for line in file:
            line = line.strip()
            if line.startswith("- ") and not line.startswith("- [[") and not line.startswith("- `") and not line.startswith("- Total"):
                all_text.add(line)

for text in sorted(all_text):
    print(text)
