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
    #[derive(Debug, Clone)]
    pub struct CommandParseError;

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

    impl SubmarineCommand {
        pub fn parse(text: &str) -> Result<SubmarineCommand, CommandParseError> {
            match text.trim().split_whitespace().collect::<Vec<&str>>() {
                x if x.len() == 2 => {
                    x[1].parse::<i32>()
                        .map(|n| SubmarineCommand(x[0].to_string(), n))
                        .map_err(|_| CommandParseError)
                },
                _ => Err(CommandParseError),
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
