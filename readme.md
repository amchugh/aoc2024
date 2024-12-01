# Super simple framework

## Adding a day

Steps to add a day, where `NUMBER` should be replaced with the day number:
1. duplicate `src/days/blankday.rs` to `src/days/dayNUMBER.rs`.
2. go into the newly created file and change every occurance of `DayNUMBER` with the correct number.
3. go into `src/days.rs` and add `mod dayNUMBER.rs` under the comment `// ADD_MOD_HERE`
4. Add `Box::new(dayNUMBER::DayNUMBER::new()),` under the comment `// ADD_SOLUTION_HERE`
5. Copy your problem input into `data/dayNUMBER.txt`
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

You can do one of the following
```
Today with the puzzle input:    $ cargo run
Today with custom input:        $ cargo run -- path/to/input.txt
Day number 4 with puzzle input: $ cargo run -- 4
Day number 6 with custom input: $ cargo run -- 6 path/to/input.txt
Running all puzzles:            $ cargo run -- all
```

Once done, you can run the following to build and run the optimized version:
```
cargo run -r -- all
```
