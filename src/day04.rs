use advent_of_code_2019::*;

fn main() -> Result<()> {
    let range = 234208..=765869_i32;

    let count = range.filter(|i| Digits::from(*i).is_possible_pwd())
        .count();
        
    answer!(04, 1, count);
    Ok(())
}

#[derive(Debug)]
struct Digits {
    digits: Vec<Digit>,
    index: usize,
}

impl Digits {
    fn is_possible_pwd(&self) -> bool {
        self.non_decreasing() && self.has_double()
    }

    fn non_decreasing(&self) -> bool {
        let mut peekable = self.digits.iter().peekable();
        while let Some(digit) = peekable.next() {
            if let Some(peek) = peekable.peek() {
                if digit < peek {
                    return false
                }
            }
        }

        true
    }

    fn has_double(&self) -> bool {
        let mut peekable = self.digits.iter().peekable();
        while let Some(digit) = peekable.next() {
            if let Some(peek) = peekable.peek() {
                if &digit == peek {
                    return true
                }
            }
        }

       false
    }
}

#[test]
fn has_double() {
    assert!(Digits::from(111111).is_possible_pwd());
    assert!(!Digits::from(223450).is_possible_pwd());
    assert!(!Digits::from(123789).is_possible_pwd());
}

impl Iterator for Digits {
    type Item = Digit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.digits.len() {
            self.index += 1;
            Some(self.digits[self.index - 1])
        } else {
            None
        }
    }
}

impl From<i32> for Digits {
    fn from(i: i32) -> Self {
        let digits = i.to_string()
            .chars()
            .map(|c| Digit::from(c))
            .collect();

        Digits {
            digits, index: 0
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Digit {
    Zero  = 0,
    One   = 1,
    Two   = 2,
    Three = 3,
    Four  = 4,
    Five  = 5,
    Six   = 6,
    Seven = 7,
    Eight = 8,
    Nine  = 9,
}

impl From<char> for Digit {
    fn from(c: char) -> Self {
        match c {
            '0' => Digit::Zero,
            '1' => Digit::One,
            '2' => Digit::Two,
            '3' => Digit::Three,
            '4' => Digit::Four,
            '5' => Digit::Five,
            '6' => Digit::Six,
            '7' => Digit::Seven,
            '8' => Digit::Eight,
            '9' => Digit::Nine,
            _   => panic!("You can't use a non-digit character!: {}", c),
        }
    }
}

#[test]
fn from_i32() {
    let i = Digits::from(123456);
}
