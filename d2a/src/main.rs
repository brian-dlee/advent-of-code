use y2021::utils;

#[derive(Debug, Clone)]
struct CommandParseError;

struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn zero() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }
}

struct SubmarineCommand(String, i32);

impl SubmarineCommand {
    fn parse(text: &str) -> Result<SubmarineCommand, CommandParseError> {
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

fn main() {
    println!("Starting Day 2a");

    let contents = utils::read_input("../input/d2a/input.txt");
    let commands: Vec<SubmarineCommand> = contents.lines().map(SubmarineCommand::parse)
        .map(|r| r.unwrap()).collect();
    let mut position = Position::zero();

    println!("Following commands to final position.");

    for command in commands {
        match command.0.as_str() {
            "forward" => position.horizontal += command.1,
            "up" => position.depth -= command.1,
            "down" => position.depth += command.1,
            _ => panic!("Unknown direction supplied: {}", command.0),
        }
    }

    println!("Final position: horizontal={}, depth={}", position.horizontal, position.depth);
    println!("Final solution: {}", position.horizontal * position.depth);
}
