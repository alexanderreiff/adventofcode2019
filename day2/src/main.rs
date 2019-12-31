mod gravity_assist;
mod intcode;

use std::fs;
use std::io::Error;

fn main() {
    let init_state = input_set().unwrap();
    let (noun, verb) = gravity_assist::input_pair_for_output(&init_state, 19_690_720)
        .expect("No matching input pairs :(");
    let result = 100 * noun + verb;
    println!("Result:\t{}", result);
    println!("Noun:\t{}\tVerb:\t{}", noun, verb);
}

fn input_set() -> Result<Vec<u32>, Error> {
    let input = fs::read_to_string("data/init_state.txt")?;
    let values = input
        .split(',')
        .filter_map(|str| str.parse().ok())
        .collect();
    Ok(values)
}
