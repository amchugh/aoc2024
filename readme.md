# Super simple framework

## Adding a day

Steps to add a day, where `NUMBER` should be replaced with the day number:
1. duplicate `src/days/blankday.rs` to `src/days/dayNUMBER.rs`.
2. go into the newly created file and change every occurance of `DayNUMBER` with the correct number.
3. go into `src/days.rs` and add `mod dayNUMBER.rs` under the comment `// ADD_MOD_HERE`
4. Add `Box::new(dayNUMBER::DayNUMBER::new()),` under the comment `// ADD_SOLUTION_HERE`
5. Copy your problem input into `data/dayNUMBER.txt`
6. Implement the `todos!` and run! 


