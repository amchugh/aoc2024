
use std::{cmp::min, collections::{HashSet, VecDeque}};

use crate::days::Solution;

const WIDTH : usize = 71;
const HEIGHT : usize = 71;

#[derive(Debug)]
pub struct Day18 {
    // State generated by `parse_input`
    falling: Vec<(usize, usize)>
}

impl Day18 {
    // Needed for creating a blank day
    pub fn new() -> Day18 {
        Day18 {
            falling: vec![],
        }
    }

    fn is_valid(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
        if x >= WIDTH || y >= HEIGHT {
            false
        } else {
            grid[y][x]
        }
    }

    fn bfs(grid: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
        let mut seen : HashSet::<(usize, usize)> = HashSet::new();
        let mut queue : VecDeque<(usize, usize, usize)> = VecDeque::new();
        queue.push_back((start.0, start.1, 0));

        while let Some(current) = queue.pop_front() {
            let x = current.0;
            let y = current.1;
            if seen.contains(&(x, y)) {
                continue;
            }
            seen.insert((x, y));
            // See if we are there
            if x == end.0 && y == end.1 {
                return Some(current.2);
            }
            // Find the future candidates
            {
                let x = x + 1;
                let y = y;
                if Self::is_valid(grid, x, y) {
                    queue.push_back((x, y, current.2 + 1));
                }
            }
            {
                let x = x - 1;
                let y = y;
                if Self::is_valid(grid, x, y) {
                    queue.push_back((x, y, current.2 + 1));
                }
            }
            {
                let x = x;
                let y = y + 1;
                if Self::is_valid(grid, x, y) {
                    queue.push_back((x, y, current.2 + 1));
                }
            }
            {
                let x = x;
                let y = y - 1;
                if Self::is_valid(grid, x, y) {
                    queue.push_back((x, y, current.2 + 1));
                }
            }
        }
        None
    }
}

impl Solution for Day18 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.falling = vec![];
    }

    fn parse_input(&mut self, file_contents: &str) {
        self.falling = file_contents.split("\n").map(|pair| {
            let mut pair = pair.split(",");
            let x = pair.next().unwrap().parse().unwrap();
            let y = pair.next().unwrap().parse().unwrap();
            (x, y)
        }).collect();
    }

    fn part1(&self) -> String {
        // Apply the first 1024
        let mut grid = vec![vec![true; WIDTH]; HEIGHT];
        let apply = min(1024, self.falling.len());
        for pos in &self.falling.as_slice()[0..apply] {
            grid[pos.1][pos.0] = false;
        }

        Self::bfs(&grid, (0, 0), (WIDTH - 1, HEIGHT - 1)).unwrap().to_string()
    }

    fn part2(&self) -> String {
        // Binary search
        let mut right = self.falling.len() - 1;
        let mut left = 1024;
        let mut grid = vec![vec![true; WIDTH]; HEIGHT];

        while right > left {
            let middle = (right + left) / 2;
            // Test it
            for pos in &self.falling.as_slice()[0..middle] {
                grid[pos.1][pos.0] = false;
            }
            let result = Self::bfs(&grid, (0, 0), (WIDTH - 1, HEIGHT - 1));
            // Reset the grid
            for pos in &self.falling.as_slice()[0..middle] {
                grid[pos.1][pos.0] = true;
            }
            // Fix our bounds
            if result.is_some() {
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }

        let pos = self.falling[right];
        let x = pos.0;
        let y = pos.1;
        format!("{},{}", x, y)
    }
}