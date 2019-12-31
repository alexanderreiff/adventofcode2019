mod fuel_calc;

use fuel_calc::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

fn main() {
    let sum = input_values().fold(0, |acc, weight| acc + fuel_calculator(weight));
    println!("Total fuel needed: {}", sum)
}

fn input_values() -> impl Iterator<Item = i32> {
    match File::open("data/input.txt") {
        Ok(file) => BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|str| str.parse().ok()),
        Err(err) => panic!(err),
    }
}
