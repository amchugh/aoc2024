use std::{collections::HashMap, fs, time::{Duration, SystemTime}};
use chrono::Datelike;

pub trait Solution {
    fn reset(&mut self);
    fn parse_input(&mut self, file_contents: &str);
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

mod aidan;
mod will;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Person {
    Aidan,
    Will
}

// -----------------------------------------------

pub fn get_solutions(by: Person) -> HashMap<usize, Box<dyn Solution>> {
    let mut result: HashMap<usize, Box<dyn Solution>> = HashMap::new();
    match by {
        Person::Will => {
            // ADD_SOLUTION_HERE
        }
        Person::Aidan => {
            result.insert(1, Box::new(aidan::day1::Day1::new()));
            result.insert(3, Box::new(aidan::day3::Day3::new()));
            result.insert(4, Box::new(aidan::day4::Day4::new()));
            result.insert(5, Box::new(aidan::day5::Day5::new()));
            result.insert(6, Box::new(aidan::day6::Day6::new()));
            result.insert(7, Box::new(aidan::day7::Day7::new()));
            result.insert(8, Box::new(aidan::day8::Day8::new()));
            result.insert(9, Box::new(aidan::day9::Day9::new()));
            result.insert(10, Box::new(aidan::day10::Day10::new()));
            result.insert(11, Box::new(aidan::day11::Day11::new()));
            result.insert(12, Box::new(aidan::day12::Day12::new()));
            result.insert(13, Box::new(aidan::day13::Day13::new()));
            result.insert(14, Box::new(aidan::day14::Day14::new()));
            result.insert(15, Box::new(aidan::day15::Day15::new()));
            result.insert(16, Box::new(aidan::day16::Day16::new()));
            result.insert(17, Box::new(aidan::day17::Day17::new()));
            result.insert(18, Box::new(aidan::day18::Day18::new()));
            result.insert(19, Box::new(aidan::day19::Day19::new()));
            result.insert(20, Box::new(aidan::day20::Day20::new()));
            result.insert(21, Box::new(aidan::day21::Day21::new()));
            result.insert(22, Box::new(aidan::day22::Day22::new()));
            result.insert(23, Box::new(aidan::day23::Day23::new()));
            result.insert(24, Box::new(aidan::day24::Day24::new()));
            result.insert(25, Box::new(aidan::day25::Day25::new()));
        }
    }
    // -----------------
    result
}

pub fn get_today() -> Option<usize> {
    // Get the current day
    let now = chrono::Local::now();
    if now.month() == 12 {
        Some(now.day() as usize)
    } else {
        None
    }
}

// -----------------------------------------------

fn read_file(filepath: &str) -> String {
    fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Failed to read data file: {}", filepath))
}

fn get_formatted_time(d: &Duration) -> String {
    if d.as_secs() > 9 {
        format!("{}s", d.as_secs())
    } else if d.as_millis() > 9 {
        format!("{}ms", d.as_millis())
    } else {
        format!("{}us", d.as_micros())
    }
}

pub fn run_many_times(sol: &mut Box<dyn Solution>, filepath: &str, times: usize) {
    let input = read_file(filepath);
    let mut average: Duration;
    let mut min_time: Duration;
    let mut max_time: Duration;

    // Run the first time
    let start: SystemTime = SystemTime::now();
    sol.reset();
    sol.parse_input(&input);
    let _ = sol.part1();
    let _ = sol.part2();
    let end = SystemTime::now();
    average = end.duration_since(start).unwrap();
    min_time = average;
    max_time = average;

    // Run many more times
    for _ in 0..times-1 {
        sol.reset();

        let start: SystemTime = SystemTime::now();
        sol.parse_input(&input);
        let _ = sol.part1();
        let _ = sol.part2();
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();

        min_time = duration.min(min_time);
        max_time = duration.max(max_time);

        average += duration;
    }

    average = average / times as u32;

    println!("Average duration: {} ({}us)", get_formatted_time(&average), average.as_micros());
    println!("Minimum duration: {} ({}us)", get_formatted_time(&min_time), min_time.as_micros());
    println!("Maximum duration: {} ({}us)", get_formatted_time(&max_time), max_time.as_micros());
}

pub fn print_answers(leading: String, sol: &mut Box<dyn Solution>, filepath: &str) -> usize {
    let (part1, part2) = get_answer(sol, filepath);
    let completed1 = part1 != "Not Implemented";
    let completed2 = part2 != "Not Implemented";
    println!("{} [{}] [{}] => {}, {}", leading, if completed1 { '*' } else { ' ' }, if completed2 { '*' } else { ' ' }, part1, part2);
    (completed1 as u8 + completed2 as u8) as usize
}

fn get_answer(sol: &mut Box<dyn Solution>, filepath: &str) -> (String, String) {
    let input = read_file(filepath);
    sol.parse_input(&input);
    let part1 = sol.part1();
    let part2 = sol.part2();
    (part1, part2)
}

pub fn run_day(sol: &mut Box<dyn Solution>, filepath: &str) {
    let input = read_file(filepath);
    // Time every part of this
    let start: SystemTime = SystemTime::now();
    sol.parse_input(&input);
    let end = SystemTime::now();
    let parse_duration = end.duration_since(start).unwrap();

    let start: SystemTime = SystemTime::now();
    let part1 = sol.part1();
    let end = SystemTime::now();
    let part1_duration = end.duration_since(start).unwrap();

    let start: SystemTime = SystemTime::now();
    let part2 = sol.part2();
    let end = SystemTime::now();
    let part2_duration = end.duration_since(start).unwrap();

    let total_time = parse_duration + part1_duration + part2_duration;

    // Report the results
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    println!("Duration: {} ({}us)", get_formatted_time(&total_time), total_time.as_micros());
    println!("|   Parsing: {}", get_formatted_time(&parse_duration));
    println!("|    Part 1: {}", get_formatted_time(&part1_duration));
    println!("|    Part 2: {}", get_formatted_time(&part2_duration));
}
