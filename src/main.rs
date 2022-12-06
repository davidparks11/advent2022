use std::time::Instant;

mod days;
mod io;
mod solution;

use days::*;
use io::*;
use solution::Solution;

use clap::ArgAction::SetTrue;
use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(long , short = 'v', action = SetTrue)]
    console: Option<bool>,
    #[arg(long, short = 'd', value_name = "1-25", value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,
    #[arg(long , short = 'e', action = SetTrue)]
    check_examples: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let to_console = args.console.unwrap_or_default();
    let input_folder = {
        if args.check_examples.unwrap_or_default() {
            InputType::EXAMPLE
        } else {
            InputType::INPUT
        }
    };

    match args.day {
        Some(day) => solve_day(day, to_console, input_folder),
        None => {
            for day in 1..=2 {
                solve_day(day, to_console, input_folder);
            }
        }
    }
}

//macro to return part 1 and 2 functions for a day module
//each function conforms to the patter fn partN(Vec<String>) -> Solution
macro_rules! use_day {
    ($day:tt) => {
        ($day::part1, $day::part2)
    };
}

fn solve_day(day: u8, to_console: bool, input_type: InputType) {
    let (part1, part2) = funcs_for_days(day);
    let file_content = read_file(input_type, day);
    let input = io::parse_lines(file_content.as_str());
    let mut start = Instant::now();
    let result1 = part1(input.clone());
    let duration1 = start.elapsed();
    start = Instant::now();
    let result2 = part2(input.clone());
    let duration2 = start.elapsed();
    if to_console {
        print_result(result1, duration1);
        print_result(result2, duration2);
    } else {
        write_result(vec![result1, result2], day, input_type)
    }
}

fn funcs_for_days(day: u8) -> (fn(Vec<String>) -> Solution, fn(Vec<String>) -> Solution) {
    match day {
        1 => use_day!(day01),
        2 => use_day!(day02),
        3 => use_day!(day03),
        _ => panic!("day not implemented"),
    }
}
