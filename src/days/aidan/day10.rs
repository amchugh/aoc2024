
use crate::days::Solution;

#[derive(Debug)]
pub struct Day10 {
    // State generated by `parse_input`
    grid: Vec<Vec<char>>,
    zero_locations: Vec<(usize, usize)>
}

impl Day10 {
    // Needed for creating a blank day
    pub fn new() -> Day10 {
        Day10 { grid: vec![], zero_locations: vec![] }
    }

    fn char_at(&self, x: usize, y: usize) -> Option<char> {
        if let Some(row) = self.grid.get(y) {
            if let Some(c) = row.get(x) {
                return Some(*c);
            }
        }
        None
    }

    fn find_adjacent(&self, x: usize, y: usize, target: usize) -> Vec<(usize, usize)> {
        let target_char: char = ('0' as u8 + target as u8).into();
        let mut result = vec![];

        if self.char_at(x+1, y).unwrap_or(0 as char) == target_char {
            result.push((x+1, y));
        }
        if self.char_at(x-1, y).unwrap_or(0 as char) == target_char {
            result.push((x-1, y));
        }
        if self.char_at(x, y+1).unwrap_or(0 as char) == target_char {
            result.push((x, y+1));
        }
        if self.char_at(x, y-1).unwrap_or(0 as char) == target_char {
            result.push((x, y-1));
        }

        result
    } 
}

impl Solution for Day10 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.grid = vec![];
        self.zero_locations = vec![];
    }

    fn parse_input(&mut self, file_contents: &str) {
        self.grid = file_contents.split("\n").map(|line| {
            line.chars().collect()
        }).collect();

        let mut zeroes = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '0' {
                    zeroes.push((x, y));
                }
            }
        }
        self.zero_locations = zeroes;
    }

    fn part1(&self) -> String {
        // Starting from all zeroes, find the number of unique 9s accessible
        self.zero_locations.iter().map(|x| {
            let mut stack = vec![*x];
            // Find every adjacent with that number
            for target in 1..9+1 {
                let mut next_stack = vec![];
                for pos in stack {
                    next_stack.extend(self.find_adjacent(pos.0, pos.1, target));
                }
                stack = next_stack;
            }
            stack.sort();
            stack.dedup();
            stack.len()
        }).sum::<usize>().to_string()
    }

    fn part2(&self) -> String {
        // Starting from all zeroes, find the number of unique 9s accessible
        self.zero_locations.iter().map(|x| {
            let mut stack = vec![*x];
            // Find every adjacent with that number
            for target in 1..9+1 {
                let mut next_stack = vec![];
                for pos in stack {
                    next_stack.extend(self.find_adjacent(pos.0, pos.1, target));
                }
                stack = next_stack;
            }
            stack.len()
        }).sum::<usize>().to_string()
    }
}
