use std::io::{self, Read};

use crate::day_01::{day_01, day_01_b};
use std::env;

mod add;
mod day_01;


fn main() -> io::Result<()>{
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    let result = match day.as_str() {
        "1" => day_01(buffer).to_string(),
        "1b" => day_01_b(buffer).to_string(),
        _ => "unknown".to_string()
    };
    println!("Result: {}", result);

    Ok(())
}
