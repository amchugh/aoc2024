use std::{fs, time::{Duration, SystemTime}};
use chrono::Datelike;

pub trait Solution {
    fn parse_input(&mut self, file_contents: &str);
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

// -----------------------------------------------

// ADD_MOD_HERE
mod day1;

pub fn get_all_days() -> Vec<Box<dyn Solution>> {
    // ADD_SOLUTION_HERE
    vec![ 
        Box::new(day1::Day1::new()),
    ]
}

pub fn get_today(override_day: Option<usize>) -> Option<usize> {
    if override_day.is_some() {
        Some(override_day.unwrap() - 1)
    } else {
        // Get the current day
        let now = chrono::Local::now();
        if now.month() == 12 {
            Some(now.day() as usize - 1)
        } else {
            None
        }
    }
}

// -----------------------------------------------

fn read_file(filepath: &str) -> String {
    fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Failed to read data file: {}", filepath))
}

fn get_formatted_time(d: &Duration) -> String {
    if d.as_secs() > 0 {
        format!("{}s", d.as_secs())
    } else if d.as_millis() > 0 {
        format!("{}ms", d.as_millis())
    } else {
        format!("{}us", d.as_micros())
    }
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
