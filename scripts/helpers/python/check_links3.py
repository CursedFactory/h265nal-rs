import os
import re

DIR = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/src_files"

for filename in os.listdir(DIR):
    if not filename.endswith('.md'):
        continue
    filepath = os.path.join(DIR, filename)
    with open(filepath, 'r') as f:
        content = f.read()
        
    # Find all [[
    count_open = content.count('[[')
    count_close = content.count(']]')
    if count_open != count_close:
        print(f"{filename}: Mismatched brackets: {count_open} open, {count_close} close")
