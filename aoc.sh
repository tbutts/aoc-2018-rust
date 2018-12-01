#!/bin/bash
# Dec 1, 2018
#
# Get the puzzle instructions & input from the Advent of Code website,
# and set up the Rust sub-crate for 2018.
#
# Usage:
#        aoc <day>
#
set -euo pipefail

# --- Configuration ---
# - Web Scraping -
SESS='<your "session" cookie goes here (its a hex-string)>'
YEAR="2018"
URL="https://adventofcode.com/${YEAR}/day"

# - Output -
DIR="days"
PUZZLE_FILE="input"
README_FILE="README.md"

require() {
    command -v "$1" &>/dev/null || { echo "Missing program: $1" && exit 1; }
}
require http      # HTTPie - https://httpie.org/
require pup       # Pup - https://github.com/ericchiang/pup
require html2text # html2text - https://github.com/aaronsw/html2text


get_day() {
    local day=$1
    http --body --check-status $URL/"$day" "Cookie:session=$SESS" \
        | pup '.day-desc, .day-success' \
        | html2text
}

get_puzzle() {
    local day=$1
    http --body --check-status $URL/"$day"/input "Cookie:session=$SESS"
}

main() {
    local day=$1
    local long_day=$(printf "%02d" "$day")
    local README_PATH="$DIR/$long_day/$README_FILE"
    local PUZZLE_PATH="$DIR/$long_day/$PUZZLE_FILE"

    if [[ -f $README_PATH ]]; then
        echo "Updating day $long_day \"$README_FILE\" file"
    else
        echo "Creating new project for day $long_day"
        cargo new --vcs none --name "aoc-2018-${long_day}" "$DIR/$long_day"
    fi
    mkdir -p "$(dirname "$README_PATH")" "$(dirname "$PUZZLE_PATH")"

    echo "-> $README_PATH"
    get_day "$day" > "$README_PATH"

    # get input if we don't already have it
    if [[ ! -f $PUZZLE_PATH ]]; then
        echo "Fetching puzzle input for day $long_day"
        echo "-> $PUZZLE_PATH"
        get_puzzle "$day" > "$PUZZLE_PATH"
    else
        echo "Puzzle input already acquired"
    fi
}

main "$@"
