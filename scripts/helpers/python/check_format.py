import os
import glob

files = glob.glob("/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/test_files/*.md")
files.append("/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/09_Test_File_Index.md")

for f in files:
    with open(f, 'r') as file:
        content = file.read()
        if "* " in content:
            print(f"Found '* ' in {f}")
        if "  -" in content:
            print(f"Found '  -' in {f}")
        if "## Purpose\n\n" in content:
            print(f"Found extra newline after Purpose in {f}")
