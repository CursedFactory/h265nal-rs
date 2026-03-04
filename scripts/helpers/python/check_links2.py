import os
import re

DIR = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/src_files"

for filename in os.listdir(DIR):
    if not filename.endswith('.md'):
        continue
    filepath = os.path.join(DIR, filename)
    with open(filepath, 'r') as f:
        content = f.read()
        
    # Find all links like [[target]]
    links = re.findall(r'\[\[(.*?)\]\]', content)
    for link in links:
        if '|' not in link:
            print(f"{filename}: Link missing pipe: {link}")
            
    # Find all links like [text](target)
    md_links = re.findall(r'\[(.*?)\]\((.*?)\)', content)
    for text, target in md_links:
        print(f"{filename}: Markdown link: [{text}]({target})")
