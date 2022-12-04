use std::io::{self, Read};

use crate::{
    day_01::{day_01, day_01_b},
    day_02::{rock_paper_scissors, rock_paper_scissors_pt2}, day_03::calc_total_of_high_priority,
};
use std::env;

mod day_01;
mod day_02;
mod day_03;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    println!("Running for {}", day);
    let result = match day.as_str() {
        "1" => day_01(buffer).to_string(),
        "1b" => day_01_b(buffer).to_string(),
        "2" => rock_paper_scissors(buffer).to_string(),
        "2b" => rock_paper_scissors_pt2(buffer).to_string(),
        "3" => calc_total_of_high_priority(buffer.as_str()).to_string(),
        _ => "unknown".to_string(),
    };
    println!("Result: {}", result);

    Ok(())
}
