import os
import re

DIR = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/src_files"

for filename in os.listdir(DIR):
    if not filename.endswith('.md'):
        continue
    filepath = os.path.join(DIR, filename)
    with open(filepath, 'r') as f:
        content = f.read()
        
    sections = {}
    heading_pattern = re.compile(r'^## (.*?)\n(.*?)(?=(^## |\Z))', re.MULTILINE | re.DOTALL)
    for match in heading_pattern.finditer(content):
        heading = match.group(1).strip()
        body = match.group(2).strip()
        sections[heading] = body
        
    for heading in ['Theory/background context']:
        if heading in sections:
            lines = sections[heading].split('\n')
            for line in lines:
                line = line.strip()
                if not line:
                    continue
                if '`' not in line:
                    pass # Theory can have text without backticks
