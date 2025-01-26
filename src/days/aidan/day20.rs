/* PROMPT:

Implement the following algorithm in Rust.
You will write the code for `Day20::paths(&self, skippable_steps: u64) -> HashMap<u64, u64>`;
Assume Day20 is a struct with the following definition:
```rs
#[derive(Debug)]
pub struct Day20 {
    // State generated by `parse_input`
    passable: Vec<Vec<bool>>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}
```
Use only the standard library. Write helper methods as needed.
Assume the fields inside Day20 are already set and rational.
This method will later be called twice, once with a value of 2 and once with a value of 20
to get the solutions for part 2 and 3 respectively. Assume part 1 is already solved, and do
not worry about calling `paths` with these values.

## Original Question

I have a problem. I am given a maze with a start and end location. The maze is a grid where every space is either passable or impassable. You can only move in the four adjacent directions; no diagonal movement is allowed. I need to answer the following questions:
1. What is the shortest path through the maze.
2. Assume that, once per path, you can bypass a single wall tile. For all the possible bypasses, I need to know how many steps the bypass saves. If the bypass makes the path longer than the shortest path, I do not need that recorded.
3. Similarly to number 2, once per path, you can bypass wall. This time, you can bypass up to 19 walls. The walls do not need to be consecutive, however they must be within a 19 step 'window'; after the first bypass, the rest of the bypasses must come within the next 18 steps following. This means you could step over a wall, walk 17 steps, then step over another wall, but you couldn't walk through the following wall. These bypass paths are determined by start and end location; two bypasses are the same if they start and end in the same location, regardless of the steps taken in-between. Again, I need to know how many steps the bypass saves.
This algorithm must be fast, so you should aggressively prune possible bypasses that would make the path longer. Please write a procedure in psuedocode that answers the questions.

## Proposed Solution

1. Find the shortest path from start to end using BFS. This solves question 1.
2. For every walkable tile in the maze, calculate the distance from that tile to the end. This is best implemented as a map from tile position to steps until the end.
3. For every walkable tile in the maze, calculate the distance from that tile to the end. This is best implemented as a map from tile position to steps from start.
4. For every walkable tile "source" in the maze, answer questions 2 and 3.
a. Look at every tile within a distance of 2 from "source". For every walkable tile within that distance, use the map to calculate the skipped path length using `distances_from_start[source] + distance(source, skip_end) + distances_from_end[skip_end]`. If this skipped path length is less than the shortest path length, record the delta in the solutions for part 2.
b. Repeat the process for every tile within a distance of 20 from "source" for part 3.
5. Return results.

*/

use std::collections::{HashMap, VecDeque};

use crate::days::Solution;

#[derive(Debug)]
pub struct Day20 {
    // State generated by `parse_input`
    passable: Vec<Vec<bool>>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

impl Day20 {
    // Needed for creating a blank day
    pub fn new() -> Day20 {
        Day20 { passable: vec![], start_pos: (0, 0), end_pos: (0, 0) }
    }

    fn paths(&self, skippable_steps: u64) -> HashMap<u64, u64> {
        // Compute the original shortest path length using BFS
        let original_length = {
            let distances = compute_distances(self.start_pos, &self.passable);
            distances[self.end_pos.0][self.end_pos.1].unwrap()
        };

        // Compute distance from start for all passable tiles
        let distance_from_start = compute_distances(self.start_pos, &self.passable);
        // Compute distance to end for all passable tiles (using BFS from end)
        let distance_from_end = compute_distances(self.end_pos, &self.passable);

        let mut result = HashMap::new();
        let rows = self.passable.len();
        if rows == 0 {
            return result;
        }
        let cols = self.passable[0].len();

        // Iterate through each tile in the grid
        for x in 0..rows {
            for y in 0..cols {
                // Only consider passable tiles as sources
                if !self.passable[x][y] {
                    continue;
                }

                // Get the distance from start to this source tile
                let Some(d_start) = distance_from_start[x][y] else { continue; };

                let k = skippable_steps as i64;

                // Iterate over all possible (dx, dy) pairs within Manhattan distance k
                for dx in -k..=k {
                    let remaining = k - dx.abs();
                    if remaining < 0 {
                        continue;
                    }

                    for dy in -remaining..=remaining {
                        // Calculate destination coordinates
                        let x_dest = x as i64 + dx;
                        let y_dest = y as i64 + dy;

                        // Check if destination is within grid bounds
                        if x_dest < 0 || y_dest < 0 {
                            continue;
                        }

                        let x_dest = x_dest as usize;
                        let y_dest = y_dest as usize;

                        if x_dest >= rows || y_dest >= cols {
                            continue;
                        }

                        // Check if destination is passable
                        if !self.passable[x_dest][y_dest] {
                            continue;
                        }

                        // Get the distance from destination to end
                        let Some(d_end) = distance_from_end[x_dest][y_dest] else { continue; };

                        // Calculate Manhattan distance between source and destination
                        let manhattan = (dx.abs() + dy.abs()) as u64;

                        // Calculate hypothetical path length
                        let hypo_length = d_start + manhattan + d_end;

                        if hypo_length < original_length {
                            let delta = original_length - hypo_length;
                            *result.entry(delta).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        result
    }
}

// Helper function to compute distances from a starting position using BFS
fn compute_distances(start: (usize, usize), grid: &[Vec<bool>]) -> Vec<Vec<Option<u64>>> {
    let rows = grid.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = grid[0].len();
    let mut distances = vec![vec![None; cols]; rows];
    let mut queue = VecDeque::new();

    distances[start.0][start.1] = Some(0);
    queue.push_back((start.0, start.1));

    while let Some((x, y)) = queue.pop_front() {
        let current_dist = distances[x][y].unwrap();

        // Check all four directions
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            // Check if the neighbor is within grid bounds
            if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                // Check if the neighbor is passable and not yet visited
                if grid[nx][ny] && distances[nx][ny].is_none() {
                    distances[nx][ny] = Some(current_dist + 1);
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    distances
}

impl Solution for Day20 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.passable = vec![];
        self.start_pos = (0, 0);
        self.end_pos = (0, 0);
    }

    fn parse_input(&mut self, file_contents: &str) {
        self.passable = file_contents.split("\n").enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| match c {
                '.' => true,
                '#' => false,
                'S' => {
                    self.start_pos = (y, x);
                    true
                },
                'E' => {
                    self.end_pos = (y, x);
                    true
                }
                _ => panic!("Unknown char")
            }).collect()
        }).collect();
    }

    fn part1(&self) -> String {
        self.paths(2).iter().filter(|(key, _)| **key >= 100).fold(0, |acc, (_, x)| acc + x).to_string()
    }

    fn part2(&self) -> String {
        self.paths(20).iter().filter(|(key, _)| **key >= 100).fold(0, |acc, (_, x)| acc + x).to_string()
    }
}
