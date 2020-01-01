use crate::movement::Movement;

pub type Path = Vec<Movement>;

pub fn path_from_str(path_str: &str) -> Path {
    path_str
        .split(',')
        .filter_map(|move_str| move_str.parse().ok())
        .collect()
}

#[cfg(test)]
mod path_tests {
    use super::*;

    #[test]
    fn it_parses_a_directions_string_correctly() {
        let directions = "L34,U75,R43,L12,L4u,X7,R2,D2,U58";
        let movements = vec![
            Movement::Left(34),
            Movement::Up(75),
            Movement::Right(43),
            Movement::Left(12),
            Movement::Right(2),
            Movement::Down(2),
            Movement::Up(58),
        ];
        assert_eq!(movements, path_from_str(directions));
    }
}
