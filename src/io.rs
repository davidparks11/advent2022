use std::fmt::Display;
use std::fs::{self, File};
use std::io::Write;
use std::time::Duration;
use std::{env, fmt};

use crate::solution::Solution;

static ANSI_ITALIC: &str = "\x1b[3m";
static ANSI_RESET: &str = "\x1b[0m";

#[derive(Copy, PartialEq)]
pub enum InputType {
    EXAMPLE,
    INPUT,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputType::EXAMPLE => write!(f, "examples"),
            InputType::INPUT => write!(f, "inputs"),
        }
    }
}

impl Clone for InputType {
    fn clone(&self) -> Self {
        match self {
            InputType::EXAMPLE => InputType::EXAMPLE,
            InputType::INPUT => InputType::INPUT,
        }
    }
}

pub fn read_file(input_type: InputType, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join(input_type.to_string())
        .join(format!("day{:02}.txt", day));

    fs::read_to_string(filepath).expect("could not open input file")
}

#[allow(dead_code)]
pub fn nums_from_lines(input: &str) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    for line in input.lines() {
        match line.parse() {
            Ok(num) => nums.push(num),
            Err(err) => {
                if err.to_string() != "cannot parse integer from empty string" {
                    println!("failed to parse line as i32: {}", err); //assume empty line means 0
                }
                nums.push(0);
            }
        }
    }
    nums
}

pub fn parse_lines<T: std::str::FromStr>(text: &str) -> Vec<T> {
    text.lines().flat_map(str::parse).collect::<Vec<T>>()
}

pub fn write_result(results: Vec<Solution>, day: u8, input_type: InputType) {
    let cwd = env::current_dir().unwrap();

    let path = {
        if input_type == InputType::EXAMPLE {
            cwd.join("results")
                .join(format!("results_example{:02}.txt", day))
        } else {
            cwd.join("results").join(format!("results{:02}.txt", day))
        }
    };

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let formatted_results: String = results.iter().map(|r| r.to_string() + "\n").collect();
    write!(file, "{}", formatted_results).expect("To be able to write results");
}

pub fn print_result<T: Display>(day: u8, part: u8, result: T, duration: Duration) {
    println!(
        "day {}, part {}: {} {}(elapsed: {:.2?}){}",
        day, part, result, ANSI_ITALIC, duration, ANSI_RESET
    );
}
