#!/bin/bash

# Advanced TODO Scanner with JSON/CSV output
# Usage: ./scripts/todo-advanced.sh [--format json|csv|text] [--author milo] [--type todo|fix]

set -e

# Get the repository root directory (where .git is located)
REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || echo "$(dirname "$(dirname "$(realpath "$0")")")")

# Default values
FORMAT="text"
AUTHOR="milo"
PATTERN="todo|fix|hack|fixme|bug"
OUTPUT_FILE=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --format|-f)
            FORMAT="$2"
            shift 2
            ;;
        --author|-a)
            AUTHOR="$2"
            shift 2
            ;;
        --type|-t)
            PATTERN="$2"
            shift 2
            ;;
        --output|-o)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --help|-h)
            echo "Advanced TODO Scanner"
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --format, -f    Output format: text, json, csv (default: text)"
            echo "  --author, -a    Author name (default: milo)"
            echo "  --type, -t      TODO types (default: todo|fix|hack|fixme|bug)"
            echo "  --output, -o    Output file (default: stdout)"
            echo "  --help, -h      Show this help"
            echo ""
            echo "Examples:"
            echo "  $0 --format json --author milo"
            echo "  $0 --format csv --output todos.csv"
            echo "  $0 --type 'urgent|critical' --format json"
            echo ""
            echo "Repository: $REPO_ROOT"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Search function
search_todos() {
    local grep_pattern="@\{($PATTERN)\}\($AUTHOR\):"
    local todos=()
    
    # Change to repository root for consistent paths
    cd "$REPO_ROOT"
    
    while IFS= read -r -d '' file; do
        if [[ -f "$file" && -r "$file" ]]; then
            while IFS=: read -r line_num line_content; do
                if [[ -n "$line_num" && -n "$line_content" ]]; then
                    # Extract todo type
                    todo_type=$(echo "$line_content" | grep -i -o -E "@\{[^}]+\}" | sed 's/@{\(.*\)}/\1/' | tr '[:upper:]' '[:lower:]')
                    
                    # Extract description (everything after the colon)
                    description=$(echo "$line_content" | sed -E 's/.*@\{[^}]+\}\([^)]+\):[[:space:]]*(.*)/\1/')
                    
                    # Get relative path from repo root
                    rel_path=$(realpath --relative-to="$REPO_ROOT" "$file")
                    
                    # Store as structured data
                    todos+=("$rel_path|$line_num|$todo_type|$description")
                fi
            done < <(grep -n -i -E "$grep_pattern" "$file" 2>/dev/null || true)
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
    
    # Output in requested format
    case $FORMAT in
        json)
            output_json "${todos[@]}"
            ;;
        csv)
            output_csv "${todos[@]}"
            ;;
        text|*)
            output_text "${todos[@]}"
            ;;
    esac
}

output_json() {
    local todos=("$@")
    local json="{\"repository\":\"$(basename "$REPO_ROOT")\",\"path\":\"$REPO_ROOT\",\"todos\":["
    local first=true
    
    for todo in "${todos[@]}"; do
        IFS='|' read -r file line type desc <<< "$todo"
        
        if [[ $first == true ]]; then
            first=false
        else
            json+=','
        fi
        
        json+="{\"file\":\"$file\",\"line\":$line,\"type\":\"$type\",\"description\":\"$(echo "$desc" | sed 's/"/\\"/g')\"}"
    done
    
    json+="],\"summary\":{\"total\":${#todos[@]},\"author\":\"$AUTHOR\",\"pattern\":\"$PATTERN\"}}"
    
    if [[ -n "$OUTPUT_FILE" ]]; then
        # Create output directory if it doesn't exist
        mkdir -p "$(dirname "$OUTPUT_FILE")"
        echo "$json" | jq '.' > "$OUTPUT_FILE"
        echo "JSON output written to: $OUTPUT_FILE"
    else
        echo "$json" | jq '.'
    fi
}

output_csv() {
    local todos=("$@")
    local output
    
    # Header
    output="File,Line,Type,Description"$'\n'
    
    # Data rows
    for todo in "${todos[@]}"; do
        IFS='|' read -r file line type desc <<< "$todo"
        output+="\"$file\",$line,\"$type\",\"$(echo "$desc" | sed 's/"/\\"/g')\""$'\n'
    done
    
    if [[ -n "$OUTPUT_FILE" ]]; then
        # Create output directory if it doesn't exist
        mkdir -p "$(dirname "$OUTPUT_FILE")"
        echo -n "$output" > "$OUTPUT_FILE"
        echo "CSV output written to: $OUTPUT_FILE (${#todos[@]} todos)"
    else
        echo -n "$output"
    fi
}

output_text() {
    local todos=("$@")
    
    echo "📋 TODO Report"
    echo "📂 Repository: $(basename "$REPO_ROOT")"
    echo "👤 Author: $AUTHOR"
    echo "🔍 Pattern: $PATTERN"
    echo "📊 Total TODOs: ${#todos[@]}"
    echo ""
    
    if [[ ${#todos[@]} -eq 0 ]]; then
        echo "🎉 No TODOs found!"
        return
    fi
    
    local current_file=""
    
    for todo in "${todos[@]}"; do
        IFS='|' read -r file line type desc <<< "$todo"
        
        if [[ "$file" != "$current_file" ]]; then
            current_file="$file"
            echo "📁 $file"
            echo "$(printf '─%.0s' {1..50})"
        fi
        
        # Color based on type
        case "$type" in
            todo) icon="📝" ;;
            fix|fixme) icon="🔧" ;;
            hack) icon="⚡" ;;
            bug) icon="🐛" ;;
            urgent) icon="🚨" ;;
            critical) icon="💥" ;;
            *) icon="📌" ;;
        esac
        
        echo "  $icon Line $line [$type]: $desc"
    done
}

# Main execution
search_todos
