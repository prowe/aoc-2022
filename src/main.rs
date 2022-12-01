use std::io::{self, Read};

use crate::day_01::day_01;
// use std::env;

mod add;
mod day_01;


fn main() -> io::Result<()>{
    // let args: Vec<String> = env::args().collect();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let result = day_01(buffer);
    println!("Result: {}", result);
    
    Ok(())
}
