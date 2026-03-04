import os
import re

DIR = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/src_files"

for filename in os.listdir(DIR):
    if not filename.endswith('.md'):
        continue
    filepath = os.path.join(DIR, filename)
    with open(filepath, 'r') as f:
        content = f.read()
        
    links = re.findall(r'\[\[(.*?)\|.*?\]\]', content)
    for target in links:
        if target.endswith('.cc') or target.endswith('.h'):
            print(f"{filename}: Target ends with extension: {target}")
        if ' ' in target:
            print(f"{filename}: Target contains space: {target}")
