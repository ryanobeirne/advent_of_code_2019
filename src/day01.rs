use std::fs::read_to_string;

fn main() {
    let sum: usize = read_to_string("input/day01.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mass| fuel(mass))
        .sum();

    println!("{}", sum);
}

fn fuel(mass: usize) -> usize {
    mass / 3 - 2
}
