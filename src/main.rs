mod problem;
mod solutions;

use problem::Problem;
use std::{env, error::Error};

use crate::solutions::D1_SecretEntrance;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    let day = args[1]
        .parse::<u8>()
        .unwrap_or_else(|value| panic!("Not a valid day : {}", value));

    let buffer = std::fs::read_to_string(format!("./inputs/{}/input", day))
        .unwrap_or_else(|_| panic!("Cannot read the input file for the day : {}", day));

    let input = buffer.lines();

    let func = match day {
        1 => D1_SecretEntrance::solve,
        _ => unimplemented!(),
    };

    let result = func(input);

    println!("{:?}", result);

    Ok(())
}
