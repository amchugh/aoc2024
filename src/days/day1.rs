
use std::{thread, time::SystemTime};

use crate::days::Solution;

#[derive(Debug)]
pub struct Day1 {
    left: Vec<u16>,
    right: Vec<u16>,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}

impl Solution for Day1 {
    fn parse_input(&mut self, file_contents: &str) {
        // Store the data in a hmap
        let lines = file_contents.split("\n");
        let pairs = lines.map(|ln| -> (u64, u64) {
            let mut parts = ln.split("   ");
            let left = parts.next().unwrap().parse::<u64>().unwrap();
            let right = parts.next().unwrap().trim().parse::<u64>().unwrap();
            (left, right)
        });

        let pairs = pairs.collect::<Vec<_>>();
        let lmax = *pairs.iter().map(|(x, _)| x).max().unwrap() as usize + 1;
        let rmax = *pairs.iter().map(|(_, x)| x).max().unwrap() as usize + 1;
        self.left = vec![0; lmax];
        self.right = vec![0; rmax];
        for (l, r) in pairs {
            self.left[l as usize] += 1;
            self.right[r as usize] += 1;
        }
    }

    fn part1(&self) -> String {
        let mut right_index = 0;
        let mut count_right = self.right[right_index];
        let mut total_distance = 0;
        for index in 0..self.left.len() {
            if self.left[index] == 0 {
                continue;
            }
            let count = self.left[index];
            // Take the #count least keys out of the right, summing their distance
            for _ in 0..(count as usize) {
                while count_right == 0 {
                    right_index += 1;
                    count_right = self.right[right_index];
                }
                count_right = count_right - 1;
                // Calculate and add the distance of the pair we just made
                let distance = if index > right_index {
                    index - right_index
                } else {
                    right_index - index
                };
                total_distance += distance;
            }
        }
        total_distance.to_string()
    }

    fn part2(&self) -> String {
        // Calculate a total similarity score by adding up 
        // each number in the left list after multiplying it 
        // by the number of times that number appears in the right list.
        let mut similarity_score = 0;
        for index in 0..self.left.len() {
            if self.left[index] == 0 {
                continue;
            }
            let appearances = *self.right.get(index).unwrap_or(&0);
            let score = index * appearances as usize;
            similarity_score += score;
        }
        similarity_score.to_string()
    }
}
