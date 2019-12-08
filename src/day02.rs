use advent_of_code_2019::*;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("input/day02.txt")?;
    let mut program = Program::from(input);
    let program2 = program.clone();
    
    program.run();

    answer!(1, 1, program.first());

    for x in 0..=99 {
        for y in 0..=99 {
            let mut program = program2.clone().change_values(x, y);
            program.run();
            if program.first() == 19690720 {
                answer!(1, 2, format!("{}{}", x, y));
                break;
            }
        }       
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Program{
    data: Vec<usize>,
    head: usize,
}

impl Program {
    fn step(&mut self) -> Option<()> {
        let output = Command {
            opcode: OpCode::from(*self.data.get(self.head)?),
            input_a: *self.data.get(*self.data.get(self.head + 1)?)?,
            input_b: *self.data.get(*self.data.get(self.head + 2)?)?,
        }.output()?;

        let output_index = *self.data.get(self.head + 3)?;
        *self.data.get_mut(output_index)? = output;

        self.head += 4;

        Some(())
    }

    fn run(&mut self) {
        while let Some(()) = self.step() {

        }
    }

    fn first(&self) -> usize {
        *self.data.get(0).unwrap()
    }

    fn change_values(mut self, x: usize, y: usize) -> Self {
        self.data[1] = x;
        self.data[2] = y;

        self
    }
}

impl From<String> for Program {
    fn from(s: String) -> Self {
        let data = s.split(',')
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();

        Program {
            data,
            head: 0,
        }
    }
}

struct Command {
    opcode: OpCode,
    input_a: usize,
    input_b: usize,
}

impl Command {
    fn output(&self) -> Option<usize> {
        match self.opcode {
            OpCode::Add => Some(self.input_a + self.input_b),
            OpCode::Mul => Some(self.input_a * self.input_b),
            OpCode::Halt => None,
        }
    }
}

impl From<[usize; 3]> for Command {
    fn from(arr: [usize; 3]) -> Self {
        Command {
            opcode: OpCode::from(arr[0]),
            input_a: arr[1],
            input_b: arr[2],
        }
    }
}

enum OpCode {
    Add,
    Mul,
    Halt,
}

impl From<usize> for OpCode {
    fn from(u: usize) -> Self {
        match u {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            99 => OpCode::Halt,
            x => unreachable!("Something went wrong!: {}", x),
        }
    }
}

#[test]
fn first_steps() -> Result<()> {
    let mut program = Program::from("1,9,10,3,2,3,11,0,99,30,40,50".to_string());
    let expected = Program::from("1,9,10,70,2,3,11,0,99,30,40,50".to_string());

    program.step().unwrap();
    assert_eq!(program.data, expected.data);

    let expected = Program::from("3500,9,10,70,2,3,11,0,99,30,40,50".to_string());
    program.step().unwrap();
    assert_eq!(program.data, expected.data);

    Ok(())
}
