
use std::collections::{HashMap, HashSet};

use crate::days::Solution;

#[derive(Debug)]
pub struct Day8 {
    // State generated by `parse_input`
    width: usize,
    height: usize,
    nodes: HashMap<char, Vec<(usize, usize)>>
}

impl Day8 {
    // Needed for creating a blank day
    pub fn new() -> Day8 {
        Day8 { width: 0, height: 0, nodes: HashMap::new() }
    }
}

impl Solution for Day8 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.width = 0;
        self.height = 0;
        self.nodes = HashMap::new();
    }

    fn parse_input(&mut self, file_contents: &str) {
        self.height = file_contents.split("\n").map(|_| 1).sum();
        self.width = file_contents.split("\n").next().unwrap().len();

        for (y, line) in file_contents.split("\n").enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                if let Some(positions) = self.nodes.get_mut(&c) {
                    positions.push((x, y));
                } else {
                    self.nodes.insert(c, vec![(x, y)]);
                }
            }
        }

        // dbg!(self.width, self.height, &self.nodes);
    }

    fn part1(&self) -> String {
        let mut antinodes = HashSet::new();

        for (_, positions) in &self.nodes {
            assert!(positions.len() > 1);
            for idx in 0..positions.len() {
                let a = &positions[idx];
                for other_idx in idx + 1..positions.len() {
                    let b = &positions[other_idx];
                    // Figure out their reflections
                    let dx = a.0 - b.0;
                    let dy = a.1 - b.1;
                    // Attempt insert
                    if a.0 + dx < self.width && a.1 + dy < self.height {
                        antinodes.insert((a.0 + dx, a.1 + dy));
                    }
                    if b.0 - dx < self.width && b.1 - dy < self.height {
                        antinodes.insert((b.0 - dx, b.1 - dy));
                    }
                }
            }
        }

        antinodes.len().to_string()
    }

    fn part2(&self) -> String {
        let mut antinodes = HashSet::new();

        for (_, positions) in &self.nodes {
            assert!(positions.len() > 1);
            for idx in 0..positions.len() {
                let a = &positions[idx];
                for other_idx in idx + 1..positions.len() {
                    let b = &positions[other_idx];
                    // Figure out their reflections
                    let dx = a.0 - b.0;
                    let dy = a.1 - b.1;
                    for i in 0..self.width {
                        // Attempt insert
                        let dx = dx * i;
                        let dy = dy * i;
                        if a.0 + dx < self.width && a.1 + dy < self.height {
                            antinodes.insert((a.0 + dx, a.1 + dy));
                        } else {
                            break
                        }
                    }
                    for i in 0..self.width {
                        // Attempt insert
                        let dx = dx * i;
                        let dy = dy * i;
                        if b.0 - dx < self.width && b.1 - dy < self.height {
                            antinodes.insert((b.0 - dx, b.1 - dy));
                        } else {
                            break
                        }
                    }
                }
            }
        }

        antinodes.len().to_string()
    }
}
