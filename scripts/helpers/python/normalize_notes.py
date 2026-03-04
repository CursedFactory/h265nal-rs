import os
import glob
import re

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Remove previously added TL;DR and Theory sections
    content = re.sub(r'\n## TL;DR\n.*?(?=\n## |\Z)', '', content, flags=re.DOTALL)
    content = re.sub(r'\n## Theory and background\n.*?(?=\n## |\Z)', '', content, flags=re.DOTALL)

    # Find title
    title_match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
    if not title_match:
        return False, "no title found"
    
    title = title_match.group(1)
    title_end = title_match.end()

    is_index = os.path.basename(filepath) == "00_Index.md"
    is_test = "test_files" in filepath
    is_src = "src_files" in filepath
    is_type = "types" in filepath

    if is_index:
        folder_name = os.path.basename(os.path.dirname(filepath))
        tldr = f"Index of {folder_name} types."
        theory = f"This index provides a structured overview of the types defined in the {folder_name} module, mapping out the parser engineering components."
        
        new_sections = f"\n\n## TL;DR\n{tldr}\n\n## Theory and background\n{theory}"
        
        new_content = content[:title_end] + new_sections + content[title_end:]
        with open(filepath, 'w') as f:
            f.write(new_content)
        return True, "index"

    # Find purpose
    purpose_match = re.search(r'\n##\s+Purpose(?: in this repo)?\s*\n(.*?)(?=\n##\s|\Z)', content, re.MULTILINE | re.DOTALL)
    
    if not purpose_match:
        return False, "no purpose found"

    purpose_text = purpose_match.group(1).strip()
    
    # Remove the Purpose section from content
    content = content[:purpose_match.start()] + content[purpose_match.end():]
    
    # Clean purpose text
    bullets = [line.strip().lstrip('-').strip() for line in purpose_text.split('\n') if line.strip()]
    cleaned_purpose = " ".join(bullets)
    
    # Lowercase first letter if it's not an acronym
    if cleaned_purpose and cleaned_purpose[0].isupper() and (len(cleaned_purpose) == 1 or not cleaned_purpose[1].isupper()):
        cleaned_purpose = cleaned_purpose[0].lower() + cleaned_purpose[1:]

    # Generate TL;DR and Theory
    if is_test:
        tldr = "Unit tests verifying parser behavior and edge cases for the associated H.265 module."
        theory = "This file contains unit tests designed to validate the correctness of the parser implementation. It exercises both standard parsing paths and selected edge cases to ensure robust handling of the bitstream."
    else:
        # For src and types
        if cleaned_purpose.endswith('.'):
            cleaned_purpose = cleaned_purpose[:-1]
            
        tldr = f"Implements {cleaned_purpose}."
        theory = f"The primary focus of this component is {cleaned_purpose}. It provides the necessary parsing logic and state management to process this aspect of the H.265 bitstream, ensuring correct interpretation of the syntax elements."

    new_sections = f"\n\n## TL;DR\n{tldr}\n\n## Theory and background\n{theory}"
    
    # Insert after title
    # We need to find the title again because content length changed
    title_match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
    title_end = title_match.end()
    new_content = content[:title_end] + new_sections + content[title_end:]
    
    with open(filepath, 'w') as f:
        f.write(new_content)
    
    return True, "standard"

def main():
    base_dir = "/Users/alexander.carter/Documents/Zipline Notes/Research/h265nal-rs-map"
    
    targets = [
        os.path.join(base_dir, "src_files", "*.md"),
        os.path.join(base_dir, "test_files", "*.md"),
        os.path.join(base_dir, "types", "**", "*.md")
    ]
    
    files = []
    for target in targets:
        files.extend(glob.glob(target, recursive=True))
        
    stats = {
        "src_files": 0,
        "test_files": 0,
        "types": 0,
        "anomalies": []
    }
    
    for filepath in files:
        try:
            success, reason = process_file(filepath)
            if success:
                if "src_files" in filepath:
                    stats["src_files"] += 1
                elif "test_files" in filepath:
                    stats["test_files"] += 1
                elif "types" in filepath:
                    stats["types"] += 1
            else:
                stats["anomalies"].append(f"{os.path.basename(filepath)}: {reason}")
        except Exception as e:
            stats["anomalies"].append(f"{os.path.basename(filepath)}: Error - {str(e)}")
            
    print(f"src_files touched: {stats['src_files']}")
    print(f"test_files touched: {stats['test_files']}")
    print(f"types touched: {stats['types']}")
    print("Anomalies:")
    for anomaly in stats["anomalies"]:
        print(f"  - {anomaly}")

if __name__ == "__main__":
    main()
