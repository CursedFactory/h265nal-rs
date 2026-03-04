import os
import glob
import re

def fix_index(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Count the number of links in the file
    # Links look like [[types/...]]
    links = re.findall(r'\[\[.*?\]\]', content)
    count = len(links)

    # If there are no links, maybe it's the root index?
    # Let's check if it already has "- Total notes:"
    if "- Total notes:" in content:
        return

    # Insert "- Total notes: **{count}**" right after the Theory and background section
    # Or right after the title? The original had it right after the title.
    # Let's put it right after the title, before TL;DR.
    
    title_match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
    if not title_match:
        return
        
    title_end = title_match.end()
    
    # Check if it's an index file
    if os.path.basename(filepath) == "00_Index.md":
        # We only want to add this if it's a type index or similar that had it.
        # Let's just add it before TL;DR
        if count > 0:
            insertion = f"\n- Total notes: **{count}**"
            new_content = content[:title_end] + insertion + content[title_end:]
            with open(filepath, 'w') as f:
                f.write(new_content)

def main():
    base_dir = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map"
    targets = [
        os.path.join(base_dir, "types", "**", "00_Index.md")
    ]
    
    files = []
    for target in targets:
        files.extend(glob.glob(target, recursive=True))
        
    for filepath in files:
        fix_index(filepath)

if __name__ == "__main__":
    main()
