#!/bin/bash
set -e

day=$1
current_month=$(date +%m)
current_year=$(date +%Y)

if [ -z "$day" ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

if [ "$current_month" != "12" ]; then
    echo "Error: AoC puzzles only available in December"
    exit 1
fi

day_padded=$(printf "%02d" $day)
filename="day${day_padded}.ts"
input_file="day${day_padded}-input"

# Check if day is available yet
if [ "$day" -gt "$(date +%d)" ]; then
    echo "Error: Day $day puzzle not available yet"
    exit 1
fi

# Don't override existing files
if [ -f "$filename" ]; then
    echo "Warning: $filename already exists, skipping file creation"
else
    cat > "$filename" << EOF
const dayInput = await Bun.file("${input_file}").text();

let part = 1;
let example = true;
//example = false;

let part1Example = \`\`;

// leave blank if same as part1
let part2Example = \`\`;

function partOne(input: string) {
}

function partTwo(input: string) {
}


if (part === 1) console.log(partOne(example ? part1Example : dayInput));
else console.log(partTwo(example ? (part2Example.length === 0 ? part1Example : part2Example) : dayInput));
EOF
fi

# Download input if it doesn't exist
if [ ! -f "$input_file" ]; then
    aoc download -o --input-file "$input_file" || {
        echo "Error: Failed to download input file"
        rm -f "$filename"  # Cleanup if we created the TS file
        exit 1
    }
fi

rustrover "$filename" "$input_file" &
bun run --watch "$filename"
