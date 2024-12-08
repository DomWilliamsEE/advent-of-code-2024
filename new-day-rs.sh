#!/bin/bash
set -e

day=$1
current_year=$(date +%Y)

if [ -z "$day" ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

day_padded=$(printf "%02d" $day)
cargo_dir="day${day_padded}"
input_file="src/day${day_padded}-input"

# Check if day is available yet
if [ "$day" -gt "$(date +%d)" ]; then
    echo "Error: Day $day puzzle not available yet"
    exit 1
fi

# Create new crate if it doesn't exist
if [ -d "$cargo_dir" ]; then
    echo "Warning: $cargo_dir already exists, skipping creation"
else
    cargo new "$cargo_dir"
    cd "$cargo_dir"
    
    # Create src/main.rs with template
    cat > src/main.rs << 'EOF'
use std::fs;

const PART1_EXAMPLE: &str = "";
// leave blank if same as part1
const PART2_EXAMPLE: &str = "";

fn part_one(input: &str) -> i64 { todo!() }

fn part_two(input: &str) -> i64 { todo!() }

fn main() {
    let part = 1;
    let example = true;
    //example = false;

    let input = if example {
        if part == 1 {
            PART1_EXAMPLE
        } else if !PART2_EXAMPLE.is_empty() {
            PART2_EXAMPLE
        } else {
            PART1_EXAMPLE
        }
    } else {
        fs::read_to_string("src/day'$day_padded'-input")
            .expect("Failed to read input file")
    };

    let result = if part == 1 {
        part_one(&input)
    } else {
        part_two(&input)
    };

    println!("{}", result);
}
EOF

    # Download input if it doesn't exist
    if [ ! -f "$input_file" ]; then
        aoc download -o --input-file "$input_file" --input-only || {
            echo "Error: Failed to download input file"
            cd ..
            rm -rf "$cargo_dir"
            exit 1
        }
    fi
fi

rustrover "./$cargo_dir" &
