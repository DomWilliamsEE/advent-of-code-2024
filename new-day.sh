#!/usr/bin/env bash

set -euo pipefail

# Logging setup
LOG_INFO=$(tput setaf 2)    # green
LOG_WARN=$(tput setaf 3)    # yellow
LOG_ERROR=$(tput setaf 1)   # red
RESET=$(tput sgr0)

log_info() { echo "${LOG_INFO}[INFO]${RESET} $*"; }
log_warn() { echo "${LOG_WARN}[WARN]${RESET} $*" >&2; }
log_error() { echo "${LOG_ERROR}[ERROR]${RESET} $*" >&2; }

die() { log_error "$*"; exit 1; }

# Validate arguments
[[ $# -eq 2 ]] || die "Usage: $0 <year> <day>"

year=$1
day=$2

# Validate year (2015 onwards when AOC started)
[[ $year =~ ^20[0-9]{2}$ ]] || die "Invalid year: $year (must be 20XX)"
(( year >= 2015 )) || die "Invalid year: $year (must be >= 2015)"

# Validate day (1-25)
[[ $day =~ ^[0-9]{1,2}$ ]] || die "Invalid day: $day (must be 1-25)"
(( day >= 1 && day <= 25 )) || die "Invalid day: $day (must be 1-25)"

# Zero-pad day if needed
day=$(printf "%02d" "$day")

# Setup paths
input_dir="inputs"
input_file="${input_dir}/${year}-${day}"
template_dir="solutions/aoc-YYYY-DD"
target_dir="solutions/aoc-${year}-${day}"

# Check if files already exist
[[ -f "$input_file" ]] && die "Input file already exists: $input_file"
[[ -d "$target_dir" ]] && die "Solution directory already exists: $target_dir"

# Ensure input directory exists
mkdir -p "$input_dir"

# Download input
log_info "Downloading input for year $year day $day..."
if ! aoc download -y "$year" -d "$day" -I -i "$input_file" -q; then
    die "Failed to download input"
fi

# Copy and process solution template
log_info "Setting up solution directory..."
if ! cp -r "$template_dir" "$target_dir"; then
    die "Failed to copy template directory"
fi

# Replace placeholders in Rust and TOML files
log_info "Processing template files..."
find "$target_dir" -type f \( -name "*.rs" -o -name "*.toml" \) -exec sed -i \
    -e "s/YYYY/${year}/g" \
    -e "s/DD/${day}/g" {} +

log_info "Running git add..."
git add "$target_dir"

# Create symlink to lib.rs
log_info "Creating solution symlink..."
if ! ln -s "${target_dir}/src/lib.rs" "${year}-${day}.rs"; then
    die "Failed to create solution symlink"
fi

log_info "Setup complete for AOC ${year} day ${day}"