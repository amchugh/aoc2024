#!/bin/bash

# Constants
TEMPLATE_FILE="src/days/blankday.rs"
DAYS_DIR="src/days"
DAYS_RUST_FILE="src/days.rs"
INPUT_DIR="data"
MOD_COMMENT="// ADD_MOD_HERE"
SOLUTION_COMMENT="// ADD_SOLUTION_HERE"

# Get the current day of the month
current_day=$(date +%d)

# Function to clean up any created files in case of error
cleanup() {
    day_file="${DAYS_DIR}/day${current_day}.rs"
    input_file="${INPUT_DIR}/day${current_day}.txt"

    if [ -f "$day_file" ]; then
        rm "$day_file"
    fi

    if [ -f "$input_file" ]; then
        rm "$input_file"
    fi
}

# Function to find the last day in src/days.rs
find_last_day_in_rust_file() {
    last_day=$(grep -oP 'mod day\K\d+' "$DAYS_RUST_FILE" | tail -n 1)
    if [ -z "$last_day" ]; then
        echo 0  # No days found, return 0
    else
        echo "$last_day"
    fi
}

# Function to add the new day to src/days.rs
add_to_rust_file() {
    # Find the insertion points
    mod_insert_index=$(grep -n "$MOD_COMMENT" "$DAYS_RUST_FILE" | cut -d: -f1)
    solution_insert_index=$(grep -n "$SOLUTION_COMMENT" "$DAYS_RUST_FILE" | cut -d: -f1)

    # Prepare the new mod and Box::new lines
    mod_line="mod day${current_day};"
    solution_line="        Box::new(day${current_day}::Day${current_day}::new()),"

    # Insert the mod line
    sed -i "${mod_insert_index}a $mod_line" "$DAYS_RUST_FILE"
    # Insert the Box::new line
    sed -i "${solution_insert_index}a $solution_line" "$DAYS_RUST_FILE"
}

# Function to create the new day file
create_day_file() {
    day_file="${DAYS_DIR}/day${current_day}.rs"
    cp "$TEMPLATE_FILE" "$day_file"

    # Update all occurrences of DayNUMBER to Day<current_day>
    sed -i "s/DayNUMBER/Day${current_day}/g" "$day_file"
}

# Function to create the input file
create_input_file() {
    input_file="${INPUT_DIR}/day${current_day}.txt"
    touch "$input_file"  # Create an empty file
}

# Main script execution
last_day=$(find_last_day_in_rust_file)

if [ "$last_day" -eq "$current_day" ]; then
    echo "Day $current_day already exists, nothing to do."
    exit 0
elif [ "$last_day" -ne $((current_day - 1)) ]; then
    echo "Error: Day $current_day is missing. Aborting."
    cleanup
    exit 1
fi

# Proceed with adding the new day
create_day_file
add_to_rust_file
create_input_file

echo "Successfully added Day $current_day."
