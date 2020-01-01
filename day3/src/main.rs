mod intersection_distance;
mod movement;
mod path;

use path::*;
use std::fs;
use std::io::{Error, ErrorKind};

fn main() {
    let (path1, path2) = read_input().unwrap();
    println!("Path 1: {:?}", path1);
    println!("Path 2: {:?}", path2);
}

fn read_input() -> Result<(Path, Path), Error> {
    let input = fs::read_to_string("data/paths.txt")?;
    let mut parsed: Vec<Path> = input.trim().split('\n').map(path_from_str).collect();
    let path2 = parsed.pop();
    let path1 = parsed.pop();

    match (path1, path2) {
        (Some(path1), Some(path2)) => Ok((path1, path2)),
        _ => Err(Error::new(ErrorKind::Other, "unparsable!")),
    }
}
