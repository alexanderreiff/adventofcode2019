use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, PartialEq)]
pub enum ParseMovementError {
    InvalidDirection,
    InvalidLength,
}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(move_str: &str) -> Result<Self, Self::Err> {
        let (direction, length) = move_str.split_at(1);
        let length = match length.parse() {
            Ok(value) => value,
            Err(_) => return Err(ParseMovementError::InvalidLength),
        };
        let movement = match direction {
            "U" => Self::Up(length),
            "D" => Self::Down(length),
            "L" => Self::Left(length),
            "R" => Self::Right(length),
            _ => return Err(ParseMovementError::InvalidDirection),
        };
        Ok(movement)
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    #[test]
    fn it_parses_valid_strings_correctly() {
        let up = "U2".parse();
        assert_eq!(up, Ok(Movement::Up(2)));
        let down = "D12".parse();
        assert_eq!(down, Ok(Movement::Down(12)));
        let left = "L14".parse();
        assert_eq!(left, Ok(Movement::Left(14)));
        let right = "R2".parse();
        assert_eq!(right, Ok(Movement::Right(2)));
    }

    #[test]
    fn it_errors_for_invalid_strings() {
        let bad_direction = "17".parse::<Movement>();
        assert_eq!(bad_direction, Err(ParseMovementError::InvalidDirection));
        let bad_length = "U4u".parse::<Movement>();
        assert_eq!(bad_length, Err(ParseMovementError::InvalidLength));
    }
}
