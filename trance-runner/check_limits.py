import os
import sys

MAX_LINES = 250
SRC_DIR = "src"
REPORT_FILE = "LINE_LIMITS.md"

def count_lines(filepath):
    with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
        return len(f.readlines())

def main():
    violations = []
    file_stats = []
    
    if not os.path.exists(SRC_DIR):
        print(f"Error: {SRC_DIR} directory not found.")
        sys.exit(1)
        
    for root, _, files in os.walk(SRC_DIR):
        for file in files:
            if file.endswith(".rs"):
                filepath = os.path.join(root, file)
                rel_path = os.path.relpath(filepath)
                line_count = count_lines(filepath)
                file_stats.append((rel_path, line_count))
                if line_count > MAX_LINES:
                    violations.append((rel_path, line_count))
                    
    # Sort files by name for consistent markdown output
    file_stats.sort(key=lambda x: x[0])
    
    # Generate LINE_LIMITS.md
    with open(REPORT_FILE, "w", encoding="utf-8") as f:
        f.write("# Codebase File Line Limits\n\n")
        f.write(f"This project enforces a hard limit of **{MAX_LINES} lines** per source file ")
        f.write("to ensure readability and compatibility with smaller LLMs (like Mistral and Minimax).\n\n")
        
        f.write("## Status Report\n\n")
        if violations:
            f.write("❌ **WARNING: Some files exceed the line limit.**\n\n")
        else:
            f.write("✅ **SUCCESS: All files are within limits.**\n\n")
            
        f.write("| File | Line Count | Status |\n")
        f.write("|---|---|---|\n")
        for path, count in file_stats:
            status = "❌ Exceeds limit" if count > MAX_LINES else "✅ OK"
            f.write(f"| [`{path}`]({path}) | {count} | {status} |\n")
            
    # Output to stdout
    print(f"Generated {REPORT_FILE} status report.")
    if violations:
        print("\n❌ File Limit Violations Found:")
        for path, count in violations:
            print(f"  - {path}: {count} lines (exceeds {MAX_LINES})")
        sys.exit(1)
    else:
        print(f"\n✅ All {len(file_stats)} files are under {MAX_LINES} lines.")
        sys.exit(0)

if __name__ == "__main__":
    main()