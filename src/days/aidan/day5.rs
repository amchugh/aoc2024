
use std::collections::{HashMap, HashSet};

use crate::days::Solution;

type Page = usize;

#[derive(Debug)]
pub struct Day5 {
    // State generated by `parse_input`
    updates: Vec<Vec<Page>>,
    rules: HashMap<Page, HashSet<Page>>,
}

impl Day5 {
    // Needed for creating a blank day
    pub fn new() -> Day5 {
        Day5 { 
            updates: vec![],
            rules: HashMap::new(), 
        }
    }

    // Returns the page that fails the rule check
    fn try_update(&self, update: &Vec<Page>) -> Option<(Page, Page)> {
        let mut seen: HashSet<usize> = HashSet::from_iter(update.iter().cloned());
        for page in update {
            // Check if we have rules for this page number
            if let Some(rules) = self.rules.get(page) {
                // If there are any seen that needed to come after this one, we return false
                if let Some(failure) = seen.intersection(rules).next() {
                    return Some((*failure, *page));
                }
            }
            seen.remove(page);
        }
        None
    }
}

impl Solution for Day5 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.updates = vec![];
        self.rules = HashMap::new();
    }

    fn parse_input(&mut self, file_contents: &str) {
        let mut parts = file_contents.split("\n\n");
        let rules = parts.next().unwrap();
        let updates = parts.next().unwrap(); 

        for rule in rules.split("\n") {
            let mut itr = rule.split("|");
            let before = itr.next().unwrap().parse::<Page>().unwrap();
            let after = itr.next().unwrap().parse::<Page>().unwrap();

            if self.rules.contains_key(&after) {
                self.rules.get_mut(&after).unwrap().insert(before);
            }
            else {
                let mut set = HashSet::new();
                set.insert(before);
                self.rules.insert(after, set);
            }
        }
        self.updates = updates.split("\n").map(|x| x.split(",").map(|x| x.parse::<Page>().unwrap()).collect::<Vec<Page>>()).collect();
    }

    fn part1(&self) -> String {
        let total = self.updates.iter().map(|x| {
            if self.try_update(&x).is_none() {
                // The middle value
                x[x.len() / 2] as usize
            } else {
                0
            }
        }).sum::<usize>();
        
        total.to_string()
    }

    fn part2(&self) -> String {
        // Find the incorrectly ordered ones
        let incorrectly_ordered = self.updates.iter().filter_map(|x| {
            match self.try_update(&x) {
                Some(fails) => Some((x, fails)),
                None => None
            }
        });

        let mut total = 0;
        for (pages, (before, after)) in incorrectly_ordered {
            let mut pages = pages.clone();
            let mut before = before;
            let mut after = after;
            loop {
                // The before needs to go right before the after

                // Remove the before
                let (before_idx, _) = pages.iter().enumerate().find(|(_idx, x)| **x == before).unwrap();
                let removed= pages.remove(before_idx);
                assert!(removed == before);

                // Add it right before after
                let (after_idx, _) = pages.iter().enumerate().find(|(_idx, x)| **x == after).unwrap();
                pages.insert(after_idx, before);

                match self.try_update(&pages) {
                    Some(fails) => {
                        before = fails.0;
                        after = fails.1;
                    }
                    None => break
                };
            }

            total += pages[pages.len() / 2] as usize
        }

        total.to_string()
    }
}