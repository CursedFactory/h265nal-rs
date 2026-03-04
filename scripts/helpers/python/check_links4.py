import os
import re

DIR = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/src_files"

for filename in os.listdir(DIR):
    if not filename.endswith('.md'):
        continue
    filepath = os.path.join(DIR, filename)
    with open(filepath, 'r') as f:
        content = f.read()
        
    links = re.findall(r'\[\[(.*?)\]\]', content)
    for link in links:
        if '|' not in link:
            print(f"{filename}: Link missing pipe: {link}")
