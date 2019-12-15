use advent_of_code_2019::*;
use std::fs::read_to_string;

pub fn main(input: Option<&str>) -> Result<()> {
    let input = read_to_string(input.unwrap_or("input/day05.txt"))?;

    answer!(05, 1, 42);
    Ok(())
}
