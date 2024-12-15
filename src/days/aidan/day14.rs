
use std::cmp::max;

use crate::days::Solution;

const SECONDS: usize = 100;
const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

#[derive(Debug, Clone)]
struct Robot {
    x: i64, 
    y: i64,
    vx: i64, 
    vy: i64,
}

impl Robot {
    fn get_integer_pair(part: &str) -> Option<(i64, i64)> {
        // Get the two numbers after the equal sign like
        // blahblah=XXXX,YYYYY
        // => (XXXX, YYYYY)
        let mut secondhalf = part.split("=");
        secondhalf.next()?;
        let secondhalf = secondhalf.next()?;
        let mut nums = secondhalf.split(",");
        let first = nums.next()?;
        let second = nums.next()?;
        let Ok(first) = first.parse::<i64>() else {
            return None;
        };
        let Ok(second) = second.parse::<i64>() else {
            return None;
        };
        Some((first, second))
    }

    fn from(describing: &str) -> Option<Robot> {
        let mut parts = describing.split(" ");
        let position = parts.next()?;
        if !position.starts_with("p=") {
            return None;
        }
        
        let velocity = parts.next()?;
        if !velocity.starts_with("v=") {
            return None;
        }

        let position = Robot::get_integer_pair(position)?;
        let velocity = Robot::get_integer_pair(velocity)?;

        Some(Robot {
            x: position.0,
            y: position.1,
            vx: velocity.0,
            vy: velocity.1,
        })
    }
}

#[derive(Debug)]
pub struct Day14 {
    // State generated by `parse_input`
    robots: Vec<Robot>
}

impl Day14 {
    // Needed for creating a blank day
    pub fn new() -> Day14 {
        Day14 {
            robots: vec![],
        }
    }

    fn _print_grid(robots: &Vec<Robot>) {
        let mut grid: Vec<Vec<usize>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
        for robot in robots {
            grid[robot.y as usize][robot.x as usize] += 1;
        }
        for line in grid {
            for num in line {
                if num == 0 {
                    print!(".")
                }
                else {
                    print!("{num}");
                }
            }
            println!("");
        }
    }

    fn count_quads(robots: &Vec<Robot>) -> (usize, usize, usize, usize) {
        const HORIZONTAL: i64 = WIDTH / 2;
        const VERTICAL: i64 = HEIGHT / 2;
        let mut quadrants = vec![0 as usize; 4];
        // +---+---+
        // | 0 | 1 |
        // +---+---+
        // | 2 | 3 |
        // +---+---+
        for robot in robots {
            match robot.x.cmp(&HORIZONTAL) {
                std::cmp::Ordering::Less => match robot.y.cmp(&VERTICAL) {
                    std::cmp::Ordering::Less => {quadrants[0] += 1;}
                    std::cmp::Ordering::Greater => {quadrants[1] += 1;}
                    std::cmp::Ordering::Equal => {}
                }
                std::cmp::Ordering::Greater => match robot.y.cmp(&VERTICAL) {
                    std::cmp::Ordering::Less => {quadrants[2] += 1;}
                    std::cmp::Ordering::Greater => {quadrants[3] += 1;}
                    std::cmp::Ordering::Equal => {}
                }
                std::cmp::Ordering::Equal => {}
            };
        }
        (quadrants[0], quadrants[1], quadrants[2], quadrants[3])
    }
}

impl Solution for Day14 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        self.robots = vec![];
    }

    fn parse_input(&mut self, file_contents: &str) {
        // Create the robots!
        self.robots = file_contents.split("\n").map(|line| {
            let res = Robot::from(line);
            if res.is_none() {
                println!("Failed at '{line}'")
            }
            res.unwrap()
        }).collect();
    }

    fn part1(&self) -> String {
        let mut robots = self.robots.clone();
        for robot in robots.iter_mut() {
            for _ in 0..SECONDS {
                robot.x = (robot.x + robot.vx + WIDTH) % WIDTH;
                robot.y = (robot.y + robot.vy + HEIGHT) % HEIGHT;
            }
        }

        // Day14::print_grid(&robots);
        // Count them by quadrant
        let quadrants = Day14::count_quads(&robots);
        let result = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
        result.to_string()
    }

    fn part2(&self) -> String {
        // Need a way of reducing the number we need to manually review.
        let mut robots = self.robots.clone();
        for second in 1..43081462 {
            // We want to find one that has a continuous string of non-0s at the middle.
            let mut middle_col = vec![0 as usize; HEIGHT as usize];
            for robot in robots.iter_mut() {
                robot.x = (robot.x + robot.vx + WIDTH) % WIDTH;
                robot.y = (robot.y + robot.vy + HEIGHT) % HEIGHT;
                if robot.y == WIDTH / 2 {
                    middle_col[robot.x as usize] += 1;
                }
            }
            // See if we have consequtive ones!
            let mut streak = 0;
            let mut best_streak = 0;
            for i in 1..HEIGHT as usize - 1 {
                if middle_col[i - 1] > 0 && middle_col[i] > 0 {
                    streak += 1;
                }
                else {
                    best_streak = max(streak, best_streak);
                    streak = 0;
                }
            }
            if best_streak >= 10 {
                    return second.to_string();
            }
        }

        "FAILED!".to_string()
    }
}
