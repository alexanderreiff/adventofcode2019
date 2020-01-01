use crate::movement::Movement;
#[cfg(test)]
use crate::path::*;
use std::ops::Add;
#[cfg(test)]
use std::sync::mpsc::{channel, Sender};
#[cfg(test)]
use std::thread;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Add<Movement> for Point {
    type Output = Self;

    fn add(self, movement: Movement) -> Self::Output {
        let mut dest = self;

        match movement {
            Movement::Up(length) => dest.y += length as i32,
            Movement::Down(length) => dest.y -= length as i32,
            Movement::Left(length) => dest.x -= length as i32,
            Movement::Right(length) => dest.x += length as i32,
        }

        dest
    }
}

#[derive(Clone, Copy, Debug)]
struct Line(Point, Point);

#[cfg(test)]
impl Line {
    // https://rosettacode.org/wiki/Find_the_intersection_of_two_lines#Rust
    pub fn intersection(&self, other: Self) -> Option<Point> {
        let a1 = self.1.y - self.0.y;
        let b1 = self.0.x - self.1.x;
        let c1 = a1 * self.0.x + b1 * self.0.y;

        let a2 = other.1.y - other.0.y;
        let b2 = other.0.x - other.1.x;
        let c2 = a2 * other.0.x + b2 * other.0.y;

        let delta = a1 * b2 - a2 * b1;

        if delta == 0 {
            return None;
        }

        let x = (b2 * c1 - b1 * c2) / delta;
        let y = (a1 * c2 - a2 * c1) / delta;

        Some(Point::new(x, y))
    }
}

#[cfg(test)]
pub fn distance(path1: Path, path2: Path) -> Option<u32> {
    let (tx, rx) = channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || trace_path(path1, tx1));
    thread::spawn(move || trace_path(path2, tx2));

    let mut distances = vec![];
    let mut segments1 = rx.recv().unwrap();
    let segments2 = rx.recv().unwrap();

    while let Some(segment) = segments1.pop() {
        let dist = segments2
            .iter()
            .filter_map(|seg| segment.intersection(*seg))
            .map(|inter| inter.distance())
            .filter(|dist| dist.is_positive())
            .min();
        if let Some(dist) = dist {
            distances.push(dist);
        }
    }

    distances.iter().min().map(|dist| dist.to_owned() as u32)
}

#[cfg(test)]
fn trace_path(path: Path, tx: Sender<Vec<Line>>) {
    let segments = path_segments(path);
    tx.send(segments).unwrap();
}

#[cfg(test)]
fn path_segments(path: Path) -> Vec<Line> {
    let mut point = Point::new(0, 0);
    let mut segments = vec![];

    for movement in path {
        let next_point = point + movement;
        let segment = Line(point, next_point);
        segments.push(segment);
        point = next_point;
    }

    segments
}

#[cfg(test)]
mod intersection_distance_tests {
    use super::*;

    #[test]
    fn it_finds_the_shortest_distance_to_intersection() {
        let path1 = path_from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = path_from_str("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(Some(159), distance(path1, path2));
        let path1 = path_from_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = path_from_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(Some(135), distance(path1, path2));
    }
}
