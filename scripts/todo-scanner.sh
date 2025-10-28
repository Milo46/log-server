#!/bin/bash

# TODO Scanner - Find custom TODO annotations in the repository
# Usage: ./scripts/todo-scanner.sh [pattern] [author]
# Example: ./scripts/todo-scanner.sh "todo|fix" "milo"

set -e

# Get the repository root directory (where .git is located)
REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || echo "$(dirname "$(dirname "$(realpath "$0")")")")

# Default values
DEFAULT_PATTERN="todo|fix|hack|fixme|bug"
DEFAULT_AUTHOR="milo"

# Parse arguments
PATTERN=${1:-$DEFAULT_PATTERN}
AUTHOR=${2:-$DEFAULT_AUTHOR}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_header() {
    echo -e "${BLUE}================================================${NC}"
    echo -e "${BLUE} TODO Scanner - Repository: $(basename "$REPO_ROOT")${NC}"
    echo -e "${BLUE} Searching for: @{$PATTERN}($AUTHOR)${NC}"
    echo -e "${BLUE}================================================${NC}"
    echo
}

print_file_header() {
    local rel_path=$(realpath --relative-to="$REPO_ROOT" "$1")
    echo -e "${GREEN}📁 $rel_path${NC}"
    echo -e "${GREEN}$(printf '─%.0s' {1..50})${NC}"
}

print_todo_item() {
    local line_num=$1
    local content=$2
    local todo_type=$3
    
    # Color code based on TODO type
    case "$todo_type" in
        *todo*) color=$YELLOW ;;
        *fix*|*fixme*) color=$RED ;;
        *hack*) color=$MAGENTA ;;
        *bug*) color=$RED ;;
        *) color=$CYAN ;;
    esac
    
    echo -e "  ${color}Line $line_num:${NC} $content"
}

# Main search function
search_todos() {
    local total_count=0
    local file_count=0
    
    # Create regex pattern for grep
    # Matches: @{todo|fix|hack}(author): anything
    local grep_pattern="@\{($PATTERN)\}\($AUTHOR\):"
    
    # Change to repository root for consistent paths
    cd "$REPO_ROOT"
    
    # Search in all text files, excluding common directories
    while IFS= read -r -d '' file; do
        if [[ -f "$file" && -r "$file" ]]; then
            # Check if file contains any matches
            if grep -q -i -E "$grep_pattern" "$file" 2>/dev/null; then
                file_count=$((file_count + 1))
                print_file_header "$file"
                
                # Find all matches with line numbers
                while IFS=: read -r line_num line_content; do
                    if [[ -n "$line_num" && -n "$line_content" ]]; then
                        total_count=$((total_count + 1))
                        
                        # Extract the todo type from the match
                        todo_type=$(echo "$line_content" | grep -i -o -E "@\{[^}]+\}" | tr '[:upper:]' '[:lower:]')
                        
                        print_todo_item "$line_num" "$line_content" "$todo_type"
                    fi
                done < <(grep -n -i -E "$grep_pattern" "$file" 2>/dev/null || true)
                
                echo
            fi
        fi
    done < <(find . -type f \
        ! -path "./.git/*" \
        ! -path "./target/*" \
        ! -path "./node_modules/*" \
        ! -path "./.vscode/*" \
        ! -path "./logs/*" \
        ! -path "./scripts/*" \
        ! -name "*.log" \
        ! -name "*.lock" \
        ! -name "*.tmp" \
        -print0 2>/dev/null)
    
    # Print summary
    echo -e "${BLUE}================================================${NC}"
    echo -e "${BLUE} Summary${NC}"
    echo -e "${BLUE}================================================${NC}"
    echo -e "${GREEN}📊 Total TODOs found: $total_count${NC}"
    echo -e "${GREEN}📁 Files with TODOs: $file_count${NC}"
    echo -e "${GREEN}📂 Repository: $REPO_ROOT${NC}"
    
    if [[ $total_count -eq 0 ]]; then
        echo -e "${YELLOW}🎉 No TODOs found! Your code is clean!${NC}"
    fi
    
    echo
}

# Help function
show_help() {
    echo "TODO Scanner - Find custom TODO annotations"
    echo
    echo "Usage: $0 [PATTERN] [AUTHOR]"
    echo
    echo "PATTERN: Regex pattern for TODO types (default: 'todo|fix|hack|fixme|bug')"
    echo "AUTHOR:  Author name to search for (default: 'milo')"
    echo
    echo "Examples:"
    echo "  $0                          # Search for @{todo|fix|hack|fixme|bug}(milo)"
    echo "  $0 'todo|fix' 'john'        # Search for @{todo|fix}(john)"
    echo "  $0 'urgent' 'milo'          # Search for @{urgent}(milo)"
    echo
    echo "TODO Format: @{TYPE}(AUTHOR): Description"
    echo "  @{todo}(milo): Implement user authentication"
    echo "  @{fix}(milo): Handle edge case for empty input"
    echo "  @{hack}(milo): Temporary workaround, needs refactoring"
    echo
    echo "Note: Excludes ./scripts/ directory to avoid scanning this script itself"
    echo
}

# Parse command line arguments
case "${1:-}" in
    -h|--help|help)
        show_help
        exit 0
        ;;
    *)
        print_header
        search_todos
        ;;
esac
