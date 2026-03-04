import glob

files = glob.glob("/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/test_files/*.md")
files.append("/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/09_Test_File_Index.md")

for f in files:
    with open(f, 'r') as file:
        content = file.read()
        if " \n" in content:
            print(f"Trailing whitespace in {f}")
        if "\n\n\n" in content:
            print(f"Multiple blank lines in {f}")
