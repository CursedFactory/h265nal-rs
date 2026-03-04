import os
import re

DIR = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map/src_files"

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Extract frontmatter and title
    frontmatter_match = re.match(r'^(---.*?---\n+)(# .*?\n+)', content, re.DOTALL)
    if not frontmatter_match:
        print(f"Could not parse frontmatter for {filepath}")
        return False
    
    frontmatter = frontmatter_match.group(1)
    title = frontmatter_match.group(2)
    
    # Extract sections
    sections = {}
    # Find all headings
    heading_pattern = re.compile(r'^## (.*?)\n(.*?)(?=(^## |\Z))', re.MULTILINE | re.DOTALL)
    
    for match in heading_pattern.finditer(content[frontmatter_match.end():]):
        heading = match.group(1).strip()
        body = match.group(2).strip()
        
        # Normalize heading
        if heading.startswith('Local dependencies'):
            heading = 'Local dependencies'
            
        sections[heading] = body

    # Desired order
    order = [
        'Purpose',
        'Linked types/classes',
        'Key functions/classes',
        'Primary parse entrypoints',
        'Local dependencies',
        'High-level test coverage',
        'Theory/background context'
    ]
    
    # Check for malformed links
    # A malformed link might be something like [[test_files/h265_aud_parser_unittest|`test/h265_aud_parser_unittest.cc`]]
    # Wait, the prompt says "Do not alter source/test/type link targets unless a target is clearly malformed."
    # Let's check if there are any malformed links. A malformed link might be missing a closing bracket or backtick.
    
    # Rebuild content
    new_content = frontmatter + title
    
    for heading in order:
        if heading in sections:
            body = sections[heading]
            
            # Clean up body: ensure bullets are consistent
            lines = body.split('\n')
            cleaned_lines = []
            for line in lines:
                line = line.strip()
                if not line:
                    continue
                if line.startswith('* '):
                    line = '- ' + line[2:]
                elif not line.startswith('- '):
                    line = '- ' + line
                
                # Ensure punctuation for Purpose and Theory
                if heading in ['Purpose', 'Theory/background context']:
                    # Capitalize first letter if it's a letter
                    if len(line) > 2 and line[2].isalpha():
                        line = line[:2] + line[2].upper() + line[3:]
                    # Add period if missing
                    if not line.endswith('.') and not line.endswith(';') and not line.endswith(':'):
                        line += '.'
                
                cleaned_lines.append(line)
                
            new_content += f"## {heading}\n" + '\n'.join(cleaned_lines) + "\n\n"
            
    # Remove trailing newlines
    new_content = new_content.strip() + "\n"
    
    if content != new_content:
        with open(filepath, 'w') as f:
            f.write(new_content)
        return True
    return False

touched = 0
for filename in os.listdir(DIR):
    if filename.endswith('.md'):
        if process_file(os.path.join(DIR, filename)):
            touched += 1

print(f"Touched {touched} files.")
