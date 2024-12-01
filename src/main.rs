
use std::env;

use crate::days::*;
mod days;

fn run_all_days() {
    let mut all_days = get_all_days();
    for (i, day) in all_days.iter_mut().enumerate() {
        let day_number = i + 1;
        let filepath = format!("data/day{day_number}.txt");
        println!("Executing day {day_number} with {filepath}:");
        run_day(day, &filepath);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--help" {
        println!("Usage: aoc2024 [number|\"all\"] [filepath]");
        return;
    }
    
    let override_day;
    if args.len() > 1 {
        if args[1] == "all" {
            return run_all_days();
        } else {
            let num = args[1].parse::<usize>();
            match num {
                Ok(override_day_) => override_day = Some(override_day_),
                Err(_) => override_day = None
            };
        }
    } else {
        override_day = None;
    }

    let day_index = get_today(override_day);
    if day_index.is_none() {
        println!("Add the day you would like to run as the first command line argument or 'all'");
        return;
    }
    let day_index = day_index.unwrap();
    let mut all_days = get_all_days();
    let day = all_days.get_mut(day_index).unwrap_or_else(|| panic!("Failed to get day, make sure you've added it to the vector of solutions"));
    let day_number = day_index + 1;

    if args.len() == 1 || (args.len() == 2 && override_day.is_some()) {
        let filepath = format!("data/day{day_number}.txt");
        println!("Executing day {day_number} with {filepath}:");
        run_day(day, &filepath);
    } else if args.len() == 2 && override_day.is_none() {
        let filepath = &args[1];
        println!("Executing day {day_number} with {filepath}:");
        run_day(day, &filepath);
    } else {
        assert!(override_day.is_some());
        let filepath = &args[2];
        println!("Executing day {day_number} with {filepath}:");
        run_day(day, &filepath);
    }
}
