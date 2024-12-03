use std::io::Result;

mod days;
mod util;
use days::{day1::DayOne, day2::DayTwo};

fn main() -> Result<()> {

    println!("*****************************");
    println!("**** Advent of Code 2024 ****");
    println!("*****************************\n");

    println!("\n*********** DAY 1 ***********");
    println!("*****************************");
    DayOne::run();
    
    println!("\n*********** DAY 2 ***********");
    println!("*****************************");
    DayTwo::run();

    
    Ok(())
}
