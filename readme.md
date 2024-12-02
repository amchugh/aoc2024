# Super simple framework

## Adding a day

Steps to add a day, where `NUMBER` should be replaced with the day number:
1. Duplicate `src/days/blankday.rs` to `src/days/[will|aidan]/dayNUMBER.rs`.
2. Go into the newly created file and change every occurance of `DayNUMBER` with the correct number.
3. Go into `src/days/[will|aidan]/mod.rs` and add `mod dayNUMBER.rs` under the comment `// ADD_MOD_HERE`.
4. Go into `src/days.rs` and add `result.insert(NUMBER, Box::new([will|aidan]::dayNUMBER::DayNUMBER::new()));` under the comment `// ADD_SOLUTION_HERE` for the appropriate person.
5. Copy your problem input into `data/dayNUMBER.txt`.
6. Implement the `todos!` and run!

## Timing

I time the execution of every step. The solution is split into three different steps:
1. Parsing the input
2. Problem part 1
3. Problem part 2

If there is computation that should be shared between part 1 and part 2, right now it needs to happen while parsing the input.
We can change that in the future if that ends up being not smart.

What is not timed:
- Reading the input file from disk
- Printing the output

Everything else is timed.

## Running

You can compile with optimizations by adding `-r` to the cargo build options. Typically, run `cargo run` while working and then `cargo run -r -- -a -p > report-aidan.txt` when done.
