
use std::collections::HashSet;

use crate::days::Solution;

#[derive(Debug)]
pub struct Day12 {
    // State generated by `parse_input`
    grid: Vec<Vec<char>>
}

impl Day12 {
    // Needed for creating a blank day
    pub fn new() -> Day12 {
        Day12 {
            grid: vec![],
        }
    }

    fn get_at(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {
        let width = grid[0].len();
        if x >= width || y >= grid.len() {
            None
        } else {
            Some(grid[y][x])
        }
    }
}

impl Solution for Day12 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.grid = vec![];
    }

    fn parse_input(&mut self, file_contents: &str) {
        self.grid = file_contents.split("\n").map(|line| {
            line.chars().collect()
        }).collect()
    }

    fn part1(&self) -> String {
        let width = self.grid[0].len();

        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut total = 0;

        for y in 0..self.grid.len() {
            for x in 0..width {
                if seen.contains(&(x,y)) {
                    continue
                }
                // We are at a new block. Process it.
                let mut stack = vec![(x, y)];
                let mut size = 0;
                let mut perimeter = 0;
                let token = Self::get_at(&self.grid, x, y).unwrap();
                while let Some(current) = stack.pop() {
                    if seen.contains(&current) {
                        continue
                    }
                    // Increment size
                    size += 1;
                    // Count the perimeter and add the nexts
                    let mut edges = 4;
                    {
                        let x = current.0 + 1;
                        let y = current.1;
                        if let Some(adjacent) = Self::get_at(&self.grid, x, y) {
                            if adjacent == token {
                                edges -= 1;
                                stack.push((x, y))
                            }
                        }
                    }
                    {
                        let x = current.0 - 1;
                        let y = current.1;
                        if let Some(adjacent) = Self::get_at(&self.grid, x, y) {
                            if adjacent == token {
                                edges -= 1;
                                stack.push((x, y))
                            }
                        }
                    }
                    {
                        let x = current.0;
                        let y = current.1 + 1;
                        if let Some(adjacent) = Self::get_at(&self.grid, x, y) {
                            if adjacent == token {
                                edges -= 1;
                                stack.push((x, y))
                            }
                        }
                    }
                    {
                        let x = current.0;
                        let y = current.1 - 1;
                        if let Some(adjacent) = Self::get_at(&self.grid, x, y) {
                            if adjacent == token {
                                edges -= 1;
                                stack.push((x, y))
                            }
                        }
                    }
                    perimeter += edges;
                    // We've now seen this one
                    seen.insert(current);
                }
                total += size * perimeter
            }
        }

        total.to_string()
    }

    fn part2(&self) -> String {
        "Not Implemented".to_string()
    }
}