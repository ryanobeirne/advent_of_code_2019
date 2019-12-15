use advent_of_code_2019::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

pub fn main(input: Option<&str>) -> Result<()> {
    let input = read_to_string(input.unwrap_or("input/day04.txt"))?;
    // Puzzle input
    let range = range_from_str(&input)?;

    let count = range.clone()
        .filter(|i| Digits::from(*i).is_possible_pwd_part1())
        .count();
    answer!(04, 1, count);

    let count2 = range
        .filter(|i| Digits::from(*i).is_possible_pwd_part2())
        .count();
    answer!(04, 2, count2);

    Ok(())
}

fn range_from_str(s: &str) -> Result<RangeInclusive<i32>> {
    let split = s.trim().split('-')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    if split.len() !=2 {
        ioerr!(s.to_string())
    } else {
        Ok(split[0]..=split[1])
    }
}

#[derive(Debug)]
struct Digits {
    digits: Vec<Digit>,
    index: usize,
}

impl Digits {
    fn is_possible_pwd_part1(&self) -> bool {
        self.non_decreasing() && self.has_double()
    }

    fn is_possible_pwd_part2(&self) -> bool {
        self.non_decreasing() && self.has_double_not_more()
    }

    fn len(&self) -> usize {
        self.digits.len()
    }

    fn non_decreasing(&self) -> bool {
        let mut peekable = self.digits.iter().peekable();
        while let Some(digit) = peekable.next() {
            if let Some(peek) = peekable.peek() {
                if peek < &digit {
                    return false;
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

    fn has_double_not_more(&self) -> bool {
        // Counter for consecutive digits
        let mut counter = HashMap::new();

        // Loop through start indexes
        for start in 0..=self.len()-2 {
            let digit = self.digits[start];
            // Loop through end indexes
            for end in start+1..=self.len()-1 {
                let len = end - start + 1;
                if self.digits.iter()
                    .skip(start)
                    .take(len)
                    .all(|d| d == &digit)
                {
                    if len > *counter.entry(digit).or_insert(0) {
                        counter.insert(digit, len);
                    }
                }
            }
        }

        counter.iter()
            .any(|(_digit, count)| count == &2)
    }
}

#[test]
fn non_decreasing() {
    assert!(Digits::from(112233).non_decreasing());
    assert!(Digits::from(123444).non_decreasing());
    assert!(Digits::from(111122).non_decreasing());
}

#[test]
fn is_possible_part2() {
    assert!(Digits::from(112233).is_possible_pwd_part2());
    assert!(!Digits::from(123444).is_possible_pwd_part2());
    assert!(Digits::from(111122).is_possible_pwd_part2());
}

#[test]
fn is_possible_part1() {
    assert!(Digits::from(111111).is_possible_pwd_part1());
    assert!(!Digits::from(223450).is_possible_pwd_part1());
    assert!(!Digits::from(123789).is_possible_pwd_part1());
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
    let _i = Digits::from(0123456789);
}
