mod intcode;

use intcode::*;
use std::fs;
use std::io::Error;

fn main() {
    let mut init_state = input_set().unwrap();
    init_state[1] = 12;
    init_state[2] = 2;
    let intcode = Intcode::new(init_state);
    let new_state = intcode.exec();
    println!("Pointer 0: {}", new_state[0]);
}

fn input_set() -> Result<Vec<u32>, Error> {
    let input = fs::read_to_string("data/init_state.txt")?;
    let values = input
        .split(',')
        .filter_map(|str| str.parse().ok())
        .collect();
    Ok(values)
}
