import os
import shutil
from datetime import datetime

# Constants
TEMPLATE_FILE = "src/days/blankday.rs"
DAYS_DIR = "src/days"
DAYS_RUST_FILE = "src/days.rs"
INPUT_DIR = "data"
MOD_COMMENT = "// ADD_MOD_HERE"
SOLUTION_COMMENT = "// ADD_SOLUTION_HERE"

# Get the current day of the month
current_day = datetime.now().day

def cleanup():
    """Clean up the created files in case of failure."""
    day_file = f"{DAYS_DIR}/day{current_day}.rs"
    input_file = f"{INPUT_DIR}/day{current_day}.txt"
    if os.path.exists(day_file):
        os.remove(day_file)
    if os.path.exists(input_file):
        os.remove(input_file)

def find_last_day_in_rust_file():
    """Find the last 'mod' entry in src/days.rs."""
    with open(DAYS_RUST_FILE, 'r') as f:
        lines = f.readlines()

    mod_lines = [line.strip() for line in lines if line.strip().startswith("mod day")]
    if mod_lines:
        last_day_mod = mod_lines[-1]
        last_day_number = int(last_day_mod.split("day")[1].split(";")[0])
        return last_day_number
    return 0  # No mods yet, return 0 if no days exist

def add_to_rust_file():
    """Update src/days.rs with the new mod and solution."""
    with open(DAYS_RUST_FILE, 'r') as f:
        lines = f.readlines()

    # Find where to insert the new mod and solution
    mod_insert_index = lines.index(MOD_COMMENT) + 1
    solution_insert_index = lines.index(SOLUTION_COMMENT) + 1

    # Prepare the new mod and Box::new line
    mod_line = f"mod day{current_day};\n"
    solution_line = f"        Box::new(day{current_day}::Day{current_day}::new()),\n"

    # Insert the new mod and Box::new line
    lines.insert(mod_insert_index, mod_line)
    lines.insert(solution_insert_index, solution_line)

    # Write the updated file back
    with open(DAYS_RUST_FILE, 'w') as f:
        f.writelines(lines)

def create_day_file():
    """Create the new day file by copying the template."""
    day_file = f"{DAYS_DIR}/day{current_day}.rs"
    shutil.copy(TEMPLATE_FILE, day_file)

    # Update the DayNUMBER occurrences
    with open(day_file, 'r') as f:
        content = f.read()

    content = content.replace("DayNUMBER", f"Day{current_day}")

    with open(day_file, 'w') as f:
        f.write(content)

def create_input_file():
    """Create the input file for the day."""
    input_file = f"{INPUT_DIR}/day{current_day}.txt"
    with open(input_file, 'w') as f:
        f.write("")  # Create an empty file for the input

def main():
    # Check if there are missing days by finding the last day in src/days.rs
    last_day = find_last_day_in_rust_file()

    if last_day == current_day:
        print(f"Day {current_day} already exists, nothing to do.")
        return
    elif last_day != current_day - 1:
        print(f"Error: Day {current_day} is missing. Aborting.")
        cleanup()
        return

    # Proceed with adding the new day
    try:
        create_day_file()
        add_to_rust_file()
        create_input_file()
        print(f"Successfully added Day {current_day}.")
    except Exception as e:
        print(f"An error occurred: {e}")
        cleanup()

if __name__ == "__main__":
    main()
