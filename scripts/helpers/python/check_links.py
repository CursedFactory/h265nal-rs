import os
import re

DIR = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/src_files"

for filename in os.listdir(DIR):
    if not filename.endswith('.md'):
        continue
    filepath = os.path.join(DIR, filename)
    with open(filepath, 'r') as f:
        content = f.read()
        
    # Find all links like [[target|text]]
    links = re.findall(r'\[\[(.*?)\]\]', content)
    for link in links:
        if '|' in link:
            target, text = link.split('|', 1)
            if not text.startswith('`') or not text.endswith('`'):
                print(f"{filename}: Malformed link text (missing backticks): {link}")
            if ' ' in target:
                print(f"{filename}: Malformed link target (contains space): {link}")
        else:
            print(f"{filename}: Link missing pipe: {link}")
