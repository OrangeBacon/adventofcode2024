use std::{process::ExitCode, time::Instant};

mod day1;

/// list of all solutions
const DAYS: &[fn(&str) -> String] = &[day1::run];

/// Command line usage string
const USAGE: &str = "Usage: aoc2024 <DAY number> [data file path]";

fn main() -> ExitCode {
    let mut args = std::env::args();
    args.next();

    let Some(day) = args.next() else {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    };
    let Ok(day) = day.parse::<usize>() else {
        eprintln!("Unable to parse day number");
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    };

    if day > DAYS.len() {
        eprintln!(
            "Day number too high, maximum day supported = {}",
            DAYS.len()
        );
        return ExitCode::FAILURE;
    }

    let file = args.next().unwrap_or(format!("data/day{day}.txt"));

    let inp = match std::fs::read_to_string(&file) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Unable to read input file, path = `{file}`");
            eprintln!("Error: {err}");
            return ExitCode::FAILURE;
        }
    };

    let start = Instant::now();
    let res = (DAYS[day - 1])(&inp);
    let end = start.elapsed();

    println!("Day {day}:");
    println!("{res}");
    println!("Time: {:?}", end);

    ExitCode::SUCCESS
}
