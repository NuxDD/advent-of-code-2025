mod problem;
mod solutions;

use problem::Problem;
use std::{env, error::Error};

use crate::solutions::{
    D1_SecretEntrance, D2_GiftShop, D3_Lobby, D4_PrintingDepartment, D5_Cafeteria,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    let day = args[1]
        .parse::<u8>()
        .unwrap_or_else(|value| panic!("Not a valid day : {}", value));

    let input = std::fs::read_to_string(format!("./inputs/{}/input", day))
        .unwrap_or_else(|_| panic!("Cannot read the input file for the day : {}", day));

    let result = match day {
        1 => D1_SecretEntrance::solve,
        2 => D2_GiftShop::solve,
        3 => D3_Lobby::solve,
        4 => D4_PrintingDepartment::solve,
        5 => D5_Cafeteria::solve,
        _ => unimplemented!(),
    }(input);

    println!("{:?}", result);

    Ok(())
}
