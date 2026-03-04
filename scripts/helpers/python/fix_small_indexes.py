import os
import glob

def fix_index(folder_name):
    base_dir = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map"
    folder_path = os.path.join(base_dir, "types", folder_name)
    index_path = os.path.join(folder_path, "00_Index.md")
    
    # Get all markdown files in the folder except 00_Index.md
    files = [f for f in os.listdir(folder_path) if f.endswith('.md') and f != "00_Index.md"]
    files.sort()
    
    # Generate links
    links = []
    for f in files:
        name = f[:-3]
        links.append(f"- [[types/{folder_name}/{name}|`{name}`]]")
        
    # Read current index
    with open(index_path, 'r') as f:
        content = f.read()
        
    # Add links at the end
    if links:
        # Also add the Total notes count if it's missing
        if "- Total notes:" not in content:
            # Insert after title
            import re
            title_match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
            if title_match:
                title_end = title_match.end()
                insertion = f"\n- Total notes: **{len(links)}**"
                content = content[:title_end] + insertion + content[title_end:]
                
        content += "\n\n" + "\n".join(links) + "\n"
        
        with open(index_path, 'w') as f:
            f.write(content)

for folder in ["enums", "io", "support"]:
    fix_index(folder)
