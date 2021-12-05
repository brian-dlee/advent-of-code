pub mod submarine {
    use std::num::ParseIntError;

    #[derive(PartialEq, Debug)]
    pub enum CommandParseError {
        InvalidFormat,
        InvalidNumericValue(ParseIntError),
        InvalidDirection(String),
    }

    pub struct Position {
        pub horizontal: i32,
        pub depth: i32,
    }

    impl Position {
        pub fn zero() -> Position {
            Position {
                horizontal: 0,
                depth: 0,
            }
        }
    }

    pub enum Direction {
        Down,
        Forward,
        Up,
    }

    impl std::str::FromStr for Direction {
        type Err = CommandParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "forward" => Ok(Direction::Forward),
                "up" => Ok(Direction::Up),
                "down" => Ok(Direction::Down),
                d => Err(CommandParseError::InvalidDirection(d.to_string())),
            }
        }
    }

    pub struct SubmarineCommand(pub Direction, pub i32);

    impl std::str::FromStr for SubmarineCommand {
        type Err = CommandParseError;

        fn from_str(text: &str) -> Result<Self, Self::Err> {
            match text.trim().split_whitespace().collect::<Vec<&str>>() {
                x if x.len() == 2 => {
                    let direction = Direction::from_str(x[0])?;
                    let value = x[1].parse::<i32>().map_err(|e| CommandParseError::InvalidNumericValue(e))?;
                    Ok(SubmarineCommand(direction, value))
                },
                _ => Err(CommandParseError::InvalidFormat),
            }
        }
    }
}
