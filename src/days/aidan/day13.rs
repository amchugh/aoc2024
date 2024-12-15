use crate::days::Solution;

/*
 * This day is just matrix multiplication.
 *
 * Take the first example:
 *   Button A: X+94, Y+34
 *   Button B: X+22, Y+67
 *   Prize: X=8400, Y=5400
 *
 * This is the same as saying
 * 8400 = 94a + 22b
 * 5400 = 34a + 67b
 *
 * Or
 * Ax = b
 *
 * Where
 * A = [94 22]
 *     [34 67]
 * x = [a]
 *     [b]
 * b = [8400]
 *     [5400]
 *      
 * This means if we can find A^-1
 * we can easily compute x by taking A^-1 * b
 * If A^-1 has no solution, we can skip.
 */

#[derive(Debug)]
struct Vec2 ( f64, f64 );
#[derive(Debug)]
struct Mat2 ( f64, f64, f64, f64 );
// [ 0 2 ]
// [ 1 3 ]

impl Mat2 {
    fn from(left: &Vec2, right: &Vec2) -> Mat2 {
        Mat2 (
            left.0, left.1,
            right.0, right.1,
        )
    }

    fn inverted(&self) -> Mat2 {
        let det = 1. / (self.0 * self.3 - self.1 * self.2);
        Mat2(
            self.3 * det,
            -self.1 * det,
            -self.2 * det,
            self.0 * det
        )
        // let Ainv1 = Vec2(machine.B.1 * det, -machine.B.0 * det);
        // let Ainv2 = Vec2(-machine.A.1 * det, machine.A.0 * det);
    }

    fn multiply(&self, x: &Vec2) -> Vec2 {
        Vec2(
            self.0 * x.0 + self.2 * x.1,
            self.1 * x.0 + self.3 * x.1,
        )
    }
}

#[derive(Debug)]
struct Machine {
    a: Vec2,
    b: Vec2,
    prize_location: Vec2,
}

impl Machine {
    fn button(line: &str) -> Option<Vec2> {
        let mut parts = line.split(",");
        let first = parts.next()?;
        let second = parts.next()?;

        let mut first = first.split("+");
        first.next()?;
        let Ok(first) = first.next()?.parse::<f64>() else {
            return None;
        };

        let mut second = second.split("+");
        second.next()?;
        let Ok(second) = second.next()?.parse::<f64>() else {
            return None;
        };

        Some(Vec2(first, second))
    }

    fn prize_location(line: &str) -> Option<Vec2> {
        let mut parts = line.split(",");
        let first = parts.next()?;
        let second = parts.next()?;

        let mut first = first.split("=");
        first.next()?;
        let Ok(first) = first.next()?.parse::<f64>() else {
            return None;
        };

        let mut second = second.split("=");
        second.next()?;
        let Ok(second) = second.next()?.parse::<f64>() else {
            return None;
        };

        Some(Vec2(first, second))
    }

    fn from(input: &str) -> Option<Machine> {
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        let mut numbers = input.split("\n");
        let a = numbers.next()?;
        let b = numbers.next()?;
        let pl = numbers.next()?;

        let a = Self::button(a)?;
        let b = Self::button(b)?;
        let pl = Self::prize_location(pl)?;

        Some(Machine {
            a,
            b,
            prize_location: pl,
        })
    }

    fn solve(&self) -> Option<Vec2> {
        self.solve_for(&self.prize_location)
    }

    fn solve_for(&self, target: &Vec2) -> Option<Vec2> {
        let a = Mat2::from(&self.a, &self.b);
        let ainv = a.inverted();
        let x = ainv.multiply(&target);
        // No make sure the solution is approxamently integer
        let a = x.0;
        let aint = a.round() as i64;
        if (a - aint as f64).abs() > 0.001 {
            return None;
        }

        let b = x.1;
        let bint = b.round() as i64;
        if (b - bint as f64).abs() > 0.001 {
            return None;
        }

        Some(Vec2(aint as f64, bint as f64))
    }
}

#[derive(Debug)]
pub struct Day13 {
    machines: Vec<Machine>
}

impl Day13 {
    // Needed for creating a blank day
    pub fn new() -> Day13 {
        Day13 {
            machines: vec![]
        }
    }
}

impl Solution for Day13 {
    fn reset(&mut self) {
        // Should probably do the same thing new() does.
        todo!()
    }

    fn parse_input(&mut self, file_contents: &str) {
        self.machines = file_contents.split("\n\n").map(|x| Machine::from(x).unwrap()).collect()
    }

    fn part1(&self) -> String {
        let mut total = 0;

        for machine in &self.machines {
            if let Some(solution) = machine.solve() {
                total += (solution.0 * 3.0 + solution.1) as usize;
            }
        }

        total.to_string()
    }

    fn part2(&self) -> String {
        let mut total = 0;
        const EXTRA: f64 = 10000000000000.;

        for machine in &self.machines {
            let offset = Vec2(machine.prize_location.0 + EXTRA, machine.prize_location.1 + EXTRA);
            if let Some(solution) = machine.solve_for(&offset) {
                total += (solution.0 * 3.0 + solution.1) as usize;
            }
        }

        total.to_string()
    }
}