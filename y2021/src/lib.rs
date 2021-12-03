pub mod utils {
    use std::fs;

    pub fn read_input(file: &str) -> String {
        println!("Reading puzzle input: {}", file);

        return fs::read_to_string(file)
            .expect(format!("Unable to read the file {}", file).as_str());
    }

    pub fn transform_lines_to_integers(lines: &str) -> Vec<i32> {
        return lines.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    }
}

pub mod submarine {
    use std::num::ParseIntError;

    #[derive(PartialEq, Debug)]
    pub enum CommandParseError {
        InvalidFormat,
        InvalidNumericValue(ParseIntError),
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

    pub struct SubmarineCommand(pub String, pub i32);

    impl std::str::FromStr for SubmarineCommand {
        type Err = CommandParseError;

        fn from_str(text: &str) -> Result<Self, Self::Err> {
            match text.trim().split_whitespace().collect::<Vec<&str>>() {
                x if x.len() == 2 => {
                    x[1].parse::<i32>()
                        .map(|n| SubmarineCommand(x[0].to_string(), n))
                        .map_err(|e| CommandParseError::InvalidNumericValue(e))
                },
                _ => Err(CommandParseError::InvalidFormat),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
