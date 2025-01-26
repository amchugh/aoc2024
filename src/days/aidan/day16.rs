use std::{cmp::Reverse, collections::{HashMap, HashSet, VecDeque}};
use priority_queue::PriorityQueue;
use crate::days::Solution;

const INFINITY: usize = 10e10 as usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North, South, East, West
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
    fn translate(&self, pos: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::East => (pos.0 + 1, pos.1),
            Direction::West => (pos.0 - 1, pos.1),
        }
    }
    fn from_step(from: (usize, usize), to: (usize, usize)) -> Self {
        let dx = to.0 as isize - from.0 as isize;
        let dy = to.1 as isize - from.1 as isize;
       
        match (dx, dy) {
            (1, 0) => Direction::East,
            (-1, 0) => Direction::West,
            (0, 1) => Direction::South,
            (0, -1) => Direction::North,
            _ => panic!("Invalid step between {:?} and {:?}", from, to),
        }
    }
}

#[derive(Debug)]
pub struct Day16 {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Day16 {
    pub fn new() -> Self {
        Day16 { grid: vec![], width: 0, height: 0 }
    }

    fn blocked(&self, x: usize, y: usize) -> bool {
        self.grid[y][x] == '#'
    }
}

impl Solution for Day16 {
    fn reset(&mut self) {
        self.grid.clear();
        self.width = 0;
        self.height = 0;
    }

    fn parse_input(&mut self, contents: &str) {
        self.grid = contents.lines().map(|l| l.chars().collect()).collect();
        self.height = self.grid.len();
        self.width = self.grid[0].len();
    }
   
    fn part1(&self) -> String {
        self.calculate_path_cost(self.shortest_paths().first().unwrap()).to_string()
    }

    fn part2(&self) -> String {
        let mut on_best_path = HashSet::new();
        for path in self.shortest_paths() {
            for visited in path {
                on_best_path.insert(visited);
            }
        }
        on_best_path.len().to_string()
    }
}

impl Day16 {
    fn is_valid(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.width && pos.1 < self.height && !self.blocked(pos.0, pos.1)
    }

    fn update_state(
        &self,
        new_pos: (usize, usize),
        new_dir: Direction,
        new_cost: usize,
        current_pos: &(usize, usize),
        current_dir: &Direction,
        distances: &mut HashMap<((usize, usize), Direction), usize>,
        pq: &mut PriorityQueue<((usize, usize), Direction), Reverse<usize>>,
        predecessors: &mut HashMap<((usize, usize), Direction), Vec<((usize, usize), Direction)>>,
    ) {
        let entry = distances.entry((new_pos, new_dir.clone())).or_insert(INFINITY);
        if new_cost < *entry {
            *entry = new_cost;
            pq.push((new_pos, new_dir.clone()), Reverse(new_cost));
            predecessors.insert((new_pos, new_dir.clone()), vec![(*current_pos, current_dir.clone())]);
        } else if new_cost == *entry {
            predecessors.entry((new_pos, new_dir.clone()))
                .or_default()
                .push((*current_pos, current_dir.clone()));
        }
    }

    fn backtrack_paths(
        &self,
        current: ((usize, usize), Direction),
        predecessors: &HashMap<((usize, usize), Direction), Vec<((usize, usize), Direction)>>,
        current_path: &mut VecDeque<(usize, usize)>,
        paths: &mut Vec<Vec<(usize, usize)>>,
    ) {
        current_path.push_front(current.0);
        if current.0 == (1, self.height - 2) {
            paths.push(current_path.iter().cloned().collect());
        } else if let Some(preds) = predecessors.get(&current) {
            for pred in preds {
                self.backtrack_paths(*pred, predecessors, &mut current_path.clone(), paths);
            }
        }
    }

    fn shortest_paths(&self) -> Vec<Vec<(usize, usize)>> {
        let start = (1, self.height - 2);
        let end = (self.width - 2, 1);

        let mut distances = HashMap::new();
        let mut predecessors: HashMap<((usize, usize), Direction), Vec<((usize, usize), Direction)>> = HashMap::new();
        let mut pq = PriorityQueue::new();

        let initial_dir = Direction::East;
        pq.push((start, initial_dir.clone()), Reverse(0));
        distances.insert((start, initial_dir), 0);

        let mut end_states = HashSet::new();
        let mut min_cost = INFINITY;

        while let Some(((current_pos, current_dir), Reverse(current_cost))) = pq.pop() {
            if current_pos == end {
                if current_cost < min_cost {
                    min_cost = current_cost;
                    end_states.clear();
                    end_states.insert((current_pos, current_dir));
                } else if current_cost == min_cost {
                    end_states.insert((current_pos, current_dir));
                }
                continue;
            }

            // Forward move
            let forward_dir = current_dir.clone();
            let forward_pos = forward_dir.translate(&current_pos);
            if self.is_valid(forward_pos) {
                let new_cost = current_cost + 1;
                self.update_state(
                    forward_pos,
                    forward_dir,
                    new_cost,
                    &current_pos,
                    &current_dir,
                    &mut distances,
                    &mut pq,
                    &mut predecessors,
                );
            }

            // Turns
            let mut next_dir = current_dir.clone();
            for _ in 0..3 {
                next_dir = next_dir.next();
                let next_pos = next_dir.translate(&current_pos);
                if self.is_valid(next_pos) {
                    let new_cost = current_cost + 1001;
                    self.update_state(
                        next_pos,
                        next_dir.clone(),
                        new_cost,
                        &current_pos,
                        &current_dir,
                        &mut distances,
                        &mut pq,
                        &mut predecessors,
                    );
                }
            }
        }
        assert!(min_cost != INFINITY);

        let mut paths = Vec::new();
        for state in end_states {
            self.backtrack_paths(state, &predecessors, &mut VecDeque::new(), &mut paths);
        }

        paths
    }

    fn calculate_path_cost(&self, path: &[(usize, usize)]) -> usize {
        if path.len() < 2 {
            return 0;
        }

        let mut cost = 0;
        let mut prev_dir = Direction::East;

        for i in 0..path.len() - 1 {
            let curr_dir = Direction::from_step(path[i], path[i + 1]);
           
            cost += if curr_dir == prev_dir { 1 } else { 1001 };
            prev_dir = curr_dir;
        }

        cost
    }
}
