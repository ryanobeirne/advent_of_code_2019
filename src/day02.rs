use advent_of_code_2019::*;
use std::fs::read_to_string;

pub fn main(input: Option<&str>) -> Result<()> {
    let input = read_to_string(input.unwrap_or("input/day02.txt"))?;
    let mut program = Program::from(input);
    let program2 = program.clone();
    
    program.run();

    answer!(2, 1, program.first());

    for x in 0..=99 {
        for y in 0..=99 {
            let mut program = program2.clone().change_values(x, y);
            program.run();
            if program.first() == 19690720 {
                answer!(2, 2, format!("{}{}", x, y));
                break;
            }
        }       
    }

    Ok(())
}

