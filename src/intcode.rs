use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program{
    pub data: Vec<usize>,
    pub head: usize,
}

impl Program {
    pub fn step(&mut self) -> Option<()> {
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

    pub fn run(&mut self) {
        while let Some(()) = self.step() {

        }
    }

    pub fn first(&self) -> usize {
        *self.data.get(0).unwrap()
    }

    pub fn change_values(mut self, x: usize, y: usize) -> Self {
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

pub struct Command {
    pub opcode: OpCode,
    pub input_a: usize,
    pub input_b: usize,
}

impl Command {
    pub fn output(&self) -> Option<usize> {
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

pub enum OpCode {
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
