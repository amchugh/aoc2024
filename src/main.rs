
use std::fs;

use clap::{Parser, ValueEnum};

use crate::days::*;
mod days;

const LAST_PERSON_FILE_PATH: &str = ".last";

fn get_default_input_file_for_day(day_number: usize) -> String {
    format!("data/day{day_number}.txt")
}

fn run_all_days(by: Person, do_perf: bool, times: usize) {
    let mut all_days = get_solutions(by);
    let len = all_days.len();
    for (i, (day_number, sol)) in all_days.iter_mut().enumerate() {
        let filepath = get_default_input_file_for_day(*day_number);
        println!("Executing for day {day_number} with {filepath}:");
        run_day(sol, &filepath);
        if do_perf {
            run_many_times(sol, &filepath, times);
        }
        if i < len - 1 {
            println!();
        }
    }
}

#[derive(clap::Parser, Debug)]
struct CLI {
    #[arg(short='n', long, value_enum)]
    person: Option<Person>,
    #[arg(short, long, value_name = "INPUT FILE")]
    input: Option<std::path::PathBuf>,
    #[arg(short, long, value_name = "DAY NUMBER")]
    day: Option<usize>,
    #[arg(short, long, help = "Run all solutions")]
    all: bool,
    #[arg(short, long, help = "Run many times to get average performance")]
    performance: bool,
    #[arg(short, long, default_value = "1000", help = "Number of times to run a solution for performance")]
    times: usize,
}

fn main() -> std::io::Result<()> {
    let options = CLI::parse();

    // Store the last person used so you don't need to set it every time :)
    let person;
    if options.person.is_some() {
        person = options.person.unwrap();
        fs::write(LAST_PERSON_FILE_PATH, format!("{:#?}", person))?;
    }
    else {
        let last_person = fs::read_to_string(LAST_PERSON_FILE_PATH);
        match last_person {
            Err(_) => {
                println!("Must set `--person` the first time.");
                return Ok(());
            }
            Ok(last_person) => {
                let last_person = Person::from_str(&last_person, true);
                if last_person.is_err() {
                    println!("Failed to read the last person. Must set `--person`.");
                    return Ok(());
                }
                person = last_person.unwrap();
            }
        }
    }

    // If we're running them all, we can ignore the other inputs
    if options.all {
        run_all_days(person, options.performance, options.times);
    } else {
        let day_number = match options.day {
            Some(x) => x,
            None => match days::get_today() {
                Some(x) => x,
                None => {
                    println!("Failed to get today. Must set `--day`.");
                    return Ok(());
                }
            }
        };

        let filepath = match options.input {
            Some(path) => path.to_str().unwrap().to_owned(),
            None => get_default_input_file_for_day(day_number)
        };

        let mut all_days = get_solutions(person);
        let sol = all_days.get_mut(&day_number);
        if sol.is_none() {
            println!("Failed to find solution day {}", day_number);
            return Ok(());
        }
        let sol = sol.unwrap();

        println!("Executing day {day_number} with {filepath}:");
        run_day(sol, &filepath);
        if options.performance {
            run_many_times(sol, &filepath, options.times);
        }
    }

    Ok(())
}
