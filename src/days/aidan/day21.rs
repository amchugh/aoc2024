use std::mem::swap;

use crate::days::Solution;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DirectionalKeypad {
    Up,
    Down,
    Left,
    Right,
    A
}

impl DirectionalKeypad {
    fn position(&self) -> (u8, u8) {
        match &self {
            DirectionalKeypad::Up =>    (1, 0),
            DirectionalKeypad::A =>     (2, 0),
            DirectionalKeypad::Left =>  (0, 1),
            DirectionalKeypad::Down =>  (1, 1),
            DirectionalKeypad::Right => (2, 1),
        }
    }

    #[allow(dead_code)]
    fn char(&self) -> char {
        match &self {
            DirectionalKeypad::Up => '^',
            DirectionalKeypad::Down => 'v',
            DirectionalKeypad::Left => '<',
            DirectionalKeypad::Right => '>',
            DirectionalKeypad::A => 'A',
        }
    }
}

fn numeric_keypad(c: char) -> (u8, u8) {
    match c {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!()
    }
}

fn diagonal_path(from: (u8, u8), to: (u8, u8)) -> (Vec<DirectionalKeypad>, Vec<DirectionalKeypad>) {
    assert!(from.0 != to.0);
    assert!(from.1 != to.1);
    let mut answer1 = vec![];
    let mut answer2 = vec![];

    if from.0 > to.0 {
        answer1.append(&mut vec![DirectionalKeypad::Left; (from.0 - to.0).into()]);
    } else {
        answer1.append(&mut vec![DirectionalKeypad::Right; (to.0 - from.0).into()]);
    }

    if from.1 > to.1 {
        answer1.append(&mut vec![DirectionalKeypad::Up; (from.1 - to.1).into()]);
        answer2.append(&mut vec![DirectionalKeypad::Up; (from.1 - to.1).into()]);
    } else {
        answer1.append(&mut vec![DirectionalKeypad::Down; (to.1 - from.1).into()]);
        answer2.append(&mut vec![DirectionalKeypad::Down; (to.1 - from.1).into()]);
    }

    if from.0 > to.0 {
        answer2.append(&mut vec![DirectionalKeypad::Left; (from.0 - to.0).into()]);
    } else {
        answer2.append(&mut vec![DirectionalKeypad::Right; (to.0 - from.0).into()]);
    }

    (answer1, answer2)
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
fn keypad_path(from: (u8, u8), to: (u8, u8)) -> (Vec<DirectionalKeypad>, Option<Vec<DirectionalKeypad>>) {
    if from.0 == to.0 {
        if from.1 < to.1 {
            (vec![DirectionalKeypad::Down; (to.1 - from.1).into()], None)
        } else {
            (vec![DirectionalKeypad::Up; (from.1 - to.1).into()], None)
        }
    }
    else if from.1 == to.1 {
        if from.0 < to.0 {
            (vec![DirectionalKeypad::Right; (to.0 - from.0).into()], None)
        } else {
            (vec![DirectionalKeypad::Left; (from.0 - to.0).into()], None)
        }
    }
    else {
        if from.0 == 0 && to.1 == 3 {
            let mut answer = vec![DirectionalKeypad::Right; (to.0 - from.0).into()];
            answer.append(&mut vec![DirectionalKeypad::Down; (to.1 - from.1).into()]);
            (answer, None)
        }
        else if from.1 == 3 && to.0 == 0 {
            let mut answer = vec![DirectionalKeypad::Up; (from.1 - to.1).into()];
            answer.append(&mut vec![DirectionalKeypad::Left; (from.0 - to.0).into()]);
            (answer, None)
        }
        else {
            let (a, b) = diagonal_path(from, to);
            (a, Some(b))
        }
    }
}

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
fn path(from: (u8, u8), to: (u8, u8)) -> (Vec<DirectionalKeypad>, Option<Vec<DirectionalKeypad>>) {
    if from.0 == to.0 {
        if from.1 < to.1 {
            (vec![DirectionalKeypad::Down; (to.1 - from.1).into()], None)
        } else {
            (vec![DirectionalKeypad::Up; (from.1 - to.1).into()], None)
        }
    }
    else if from.1 == to.1 {
        if from.0 < to.0 {
            (vec![DirectionalKeypad::Right; (to.0 - from.0).into()], None)
        } else {
            (vec![DirectionalKeypad::Left; (from.0 - to.0).into()], None)
        }
    }
    else {
        if from.0 == 0 {
            assert!(to.1 == 0);
            // Go right, then up
            let mut answer = vec![DirectionalKeypad::Right; to.0.into()];
            answer.push(DirectionalKeypad::Up);
            (answer, None)
        } else if from.1 == 0 && to.0 == 0 {
            // Go down, then left
            let mut answer = vec![DirectionalKeypad::Down; 1];
            answer.append(&mut vec![DirectionalKeypad::Left; from.0.into()]);
            (answer, None)
        } else {
            let (a, b) = diagonal_path(from, to);
            (a, Some(b))
        }
    }
}

fn keypad_path_a(from: (u8, u8), to: (u8, u8)) -> (Vec<DirectionalKeypad>, Option<Vec<DirectionalKeypad>>) {
    let mut answer = keypad_path(from, to);
    answer.0.push(DirectionalKeypad::A);
    if answer.1.is_some() {
        answer.1.as_mut().unwrap().push(DirectionalKeypad::A);
    }
    answer
}

fn path_a(from: (u8, u8), to: (u8, u8)) -> (Vec<DirectionalKeypad>, Option<Vec<DirectionalKeypad>>) {
    let mut answer = path(from, to);
    answer.0.push(DirectionalKeypad::A);
    if answer.1.is_some() {
        answer.1.as_mut().unwrap().push(DirectionalKeypad::A);
    }
    answer
}

// Let's make a perfect cache for robot_path. We will make it like a tree
struct Cache {
    entries: Vec<Option<CacheNode>>
}

#[derive(Debug)]
struct CacheNode {
    next: [Option<Box<CacheNode>>; 5],
    remaining: Vec<DirectionalKeypad>,
    value: Option<u64>
}

impl CacheNode {
    fn new(key: Vec<DirectionalKeypad>, value: Option<u64>) -> CacheNode {
        CacheNode { next: [None, None, None, None, None], remaining: key, value }
    }

    fn direction_to_idx(direction: &DirectionalKeypad) -> usize {
        match direction {
            DirectionalKeypad::Up => 0,
            DirectionalKeypad::Down => 1,
            DirectionalKeypad::Left => 2,
            DirectionalKeypad::Right => 3,
            DirectionalKeypad::A => 4,
        }
    }

    fn partial_match(a: &Vec<DirectionalKeypad>, b: &Vec<DirectionalKeypad>) -> bool {
        a.len() > b.len() && &a[..b.len()] == b
    }

    fn differing_index(a: &Vec<DirectionalKeypad>, b: &Vec<DirectionalKeypad>) -> usize {
        for (idx, (a, b)) in a.iter().zip(b.iter()).enumerate() {
            if a != b {
                return idx
            }
        }
        return a.len().min(b.len())
    }

    fn insert(&mut self, mut other: CacheNode) {
        // See if there is a partial match
        if Self::partial_match(&self.remaining, &other.remaining) {
            // Unfortunately, this means we need to swap.
            swap(&mut self.value, &mut other.value);
            // Other should have no nexts
            assert!(other.next[0].is_none());
            assert!(other.next[1].is_none());
            assert!(other.next[2].is_none());
            assert!(other.next[3].is_none());
            assert!(other.next[4].is_none());
            swap(&mut self.next, &mut other.next);
            swap(&mut self.remaining, &mut other.remaining);
            // Now we can insert again and succeed.
            self.insert(other);
        }
        else if Self::partial_match(&other.remaining, &self.remaining) {
            // This means we are a proper subset of other and it makes our life very easy.
            let direction = Self::direction_to_idx(&other.remaining[self.remaining.len()]);
            other.remaining = other.remaining[self.remaining.len() + 1..].iter().map(|x| x.clone()).collect();
            if self.next[direction].is_none() {
                self.next[direction] = Some(Box::from(other));
            } else {
                self.next[direction].as_mut().unwrap().insert(other);
            }
        }
        else {
            // Find where we differ and split
            let idx = Self::differing_index(&self.remaining, &other.remaining);
            // Our path
            let direction = Self::direction_to_idx(&self.remaining[idx]);
            let remaining = self.remaining[idx + 1..].iter().cloned().collect::<Vec<DirectionalKeypad>>();
            let mut new_cache_node = CacheNode::new(remaining, self.value);
            swap(&mut new_cache_node.next, &mut self.next);
            // Insert the new cache node
            self.next[direction] = Some(Box::from(new_cache_node));
            // We no longer have value
            self.value = None;
            // And we need to update our path as well
            self.remaining.truncate(idx);

            // Other's path
            let direction2 = Self::direction_to_idx(&other.remaining[idx]);
            assert!(direction != direction2);
            let direction = direction2;
            other.remaining = other.remaining[idx + 1..].iter().cloned().collect();
            assert!(self.next[direction].is_none());
            self.next[direction] = Some(Box::from(other));
        }
    }

    fn get_in_children(&self, key: &[DirectionalKeypad]) -> Option<u64> {
        assert!(key.len() > 0);
        let first = key.first().unwrap();
        self.next[Self::direction_to_idx(first)].as_ref().map(|x| x.get(&key[1..]))?
    }

    fn get(&self, key: &[DirectionalKeypad]) -> Option<u64> {
        if *key == self.remaining {
            // Perfect match
            return self.value;
        }

        if self.remaining.len() == 0 {
            // We need to look in our children for the answer
            return self.get_in_children(key);
        }

        if key.len() > self.remaining.len() {
            // Make sure we have a partial match
            if self.remaining == key[..self.remaining.len()] {
                // It's possible one of our next's has the value:
                return self.get_in_children(&key[self.remaining.len()..]);
            }
            // Otherwise, we've gone down the wrong path. There's no reason to
            // backtrack, so we are done.
            return None;
        }

        // Don't need to bother checking if the key is shorter than remaining as none of our children are viable.
        None
    }
}

impl Cache {
    fn new(expected_depth: usize) -> Cache {
        let mut res = Cache { entries: vec![] };
        for _ in 0..expected_depth {
            res.entries.push(None);
        }
        res
    }

    fn compute(&mut self, sequence: Vec<DirectionalKeypad>, depth: usize) -> u64 {
        let answer = robot_path(self, &sequence, depth);
        let cached_solution = CacheNode::new(sequence, Some(answer));

        // Store the answer
        if self.entries[depth].is_some() {
            self.entries[depth].as_mut().unwrap().insert(cached_solution);
        }
        else {
            self.entries[depth] = Some(cached_solution);
        }

        answer
    }

    fn get(&mut self, sequence: Vec<DirectionalKeypad>, depth: usize) -> u64 {
        assert!(depth < self.entries.len());
        if self.entries[depth].is_none() {
            return self.compute(sequence, depth);
        }
        let result = self.entries[depth].as_ref().unwrap().get(&sequence);
        if result.is_none() {
            return self.compute(sequence, depth);
        }
        result.unwrap()
    }
}

fn robot_path(cache: &mut Cache, sequence: &Vec<DirectionalKeypad>, depth: usize) -> u64 {
    if depth == 0 {
        return sequence.len() as u64;
    }

    let mut starting = DirectionalKeypad::A.position();
    let mut total = 0;
    for direction in sequence {
        let end = direction.position();
        let paths = path_a(starting, end);
        if paths.1.is_some() {
            // Need to compare both
            total += cache.get(paths.0, depth - 1).min(
                cache.get(paths.1.unwrap(), depth - 1));
        } else {
            // Only 1 option, it must be the shortest.
            total += cache.get(paths.0, depth - 1);
        }
        starting = end;
    }
    total
}

fn path_length(cache: &mut Cache, sequence: &str, depth: usize) -> u64 {
    // We need to go from 'A' to sequence.chars[0]
    let mut starting = numeric_keypad('A');
    let mut total = 0;
    for direction in sequence.chars() {
        let end = numeric_keypad(direction);
        let paths = keypad_path_a(starting, end);
        if paths.1.is_some() {
            // Need to compare both
            total += cache.get(paths.0, depth - 1).min(
                cache.get(paths.1.unwrap(), depth - 1));
        } else {
            // Only 1 option, it must be the shortest.
            total += cache.get(paths.0, depth - 1);
        }
        starting = end;
    }
    total
}

#[derive(Debug)]
pub struct Day21 {
    // State generated by `parse_input`
    paths: Vec<String>,
}

impl Day21 {
    // Needed for creating a blank day
    pub fn new() -> Day21 {
        Day21 { paths: vec![] }
    }

    fn solve(&self, depth: usize) -> u64 {
        let mut cache = Cache::new(depth);
        self.paths.iter().map(|x| {
            assert!(x.ends_with("A"));
            let answer = x.strip_suffix("A").unwrap().parse::<u64>().unwrap() * path_length(&mut cache, &x, depth);
            answer
        }).sum::<u64>()
    }
}

impl Solution for Day21 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.paths = vec![];
    }

    fn parse_input(&mut self, file_contents: &str) {
        self.paths = file_contents.split("\n").map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> String {
        self.solve(3).to_string()
    }

    fn part2(&self) -> String {
        self.solve(26).to_string()
    }
}
