
use regex::Regex;

use crate::days::Solution;

#[derive(Debug)]
pub struct Day3 {
    // State generated by `parse_input`
    re: Regex,
    input: String,
}

impl Day3 {
    // Needed for creating a blank day
    pub fn new() -> Day3 {
        Day3 { 
            re: Regex::new(r"mul\((\d+),(\d+)\)").unwrap(),
            input: String::new(),
        }
    }

    fn search(&self, input: &str) -> usize {
        let matches = self.re.captures_iter(input);
        let mut total = 0;
        for m in matches {
            let a = m.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let b = m.get(2).unwrap().as_str().parse::<usize>().unwrap();
            total += a * b;
        }
        total
    }
}

impl Solution for Day3 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.input = String::new();
    }

    fn parse_input(&mut self, file_contents: &str) {
        self.input = file_contents.to_string();
    }

    fn part1(&self) -> String {
        self.search(&self.input).to_string()
    }

    fn part2(&self) -> String {
        let mut slice = &self.input[..];
        let mut total = 0;
        loop {
            let end = slice.find(r"don't()");
            if end.is_none() {
                total += self.search(slice);
                break;
            }
            let end = end.unwrap();
            let inner = &slice[..end];
            total += self.search(inner);

            let start = slice[end..].find(r"do()");
            if start.is_none() {
                break;
            }
            let start = start.unwrap();

            slice = &slice[start+end..];
        }
        total.to_string()
    }
}
