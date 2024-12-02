
use std::collections::BTreeMap;

use crate::days::Solution;

#[derive(Debug)]
pub struct Day1 {
    left: BTreeMap<u64, u64>,
    right: BTreeMap<u64, u64>,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            left: BTreeMap::new(),
            right: BTreeMap::new(),
        }
    }
}

impl Solution for Day1 {
    fn reset(&mut self) {
        self.left = BTreeMap::new();
        self.right = BTreeMap::new();
    }

    fn parse_input(&mut self, file_contents: &str) {
        // Store the data in a hmap
        let lines = file_contents.split("\n");
        let pairs = lines.map(|ln| -> (u64, u64) {
            let mut parts = ln.split("   ");
            let left = parts.next().unwrap().parse::<u64>().unwrap();
            let right = parts.next().unwrap().trim().parse::<u64>().unwrap();
            (left, right)
        });
        for (l, r) in pairs {
            match self.left.get(&l) {
                Some(current) => { self.left.insert(l, current + 1); }
                None => { self.left.insert(l, 1); }
            };
            match self.right.get(&r) {
                Some(current) => { self.right.insert(r, current + 1); }
                None => { self.right.insert(r, 1); }
            };
        }
    }

    fn part1(&self) -> String {
        let mut right = self.right.clone();
        let (mut least_right, mut count_right) = right.pop_first().unwrap();

        let mut total_distance = 0;
        for (value, count) in self.left.iter() {
            // Take the #count least keys out of the right, summing their distance
            for _ in 0..*count {
                if count_right == 0 {
                    // Get the next smallest
                    (least_right, count_right) = right.pop_first().unwrap();
                }
                count_right = count_right - 1;
                // Calculate and add the distance of the pair we just made
                let distance = if *value > least_right {
                    *value - least_right
                } else {
                    least_right - *value
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
        for (value, _) in self.left.iter() {
            let appearances = self.right.get(value).unwrap_or(&0);
            let score = value * appearances;
            similarity_score += score;
        }
        similarity_score.to_string()
    }
}
