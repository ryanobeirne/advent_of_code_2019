use advent_of_code_2019::*;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let module_masses: Vec<isize> = read_to_string("input/day01.txt")?
        .lines()
        .filter_map(|line| line.parse::<isize>().ok())
        .collect();

    let fuel: isize = module_masses.iter()
        .filter_map(|mass| calc_fuel(*mass))
        .sum();

    answer!(1, 1, &fuel);

    let total_fuel: isize = module_masses.iter()
        .map(|mass| calc_total_fuel(*mass))
        .sum();

    answer!(1, 2, &total_fuel);

    Ok(())
}

fn calc_fuel(mass: isize) -> Option<isize> {
    let m = mass / 3 - 2;

    if m <= 0 {
        None
    } else {
        Some(m)
    }
}

fn calc_total_fuel(mass: isize) -> isize {
    let mut total_fuel = calc_fuel(mass).unwrap_or(0);
    let mut extra_fuel = calc_fuel(total_fuel).unwrap_or(0);

    total_fuel += extra_fuel;

    while let Some(fuel) = calc_fuel(extra_fuel) {
        total_fuel += fuel;
        extra_fuel = fuel;
    }

    total_fuel
}

#[test]
fn total() {
    assert_eq!(calc_total_fuel(1969), 966);
}
