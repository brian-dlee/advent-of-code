use std::str::FromStr;
use y2021::utils;
use y2021::submarine;

fn main() {
    println!("Starting Day 2a");

    let contents = utils::read_input("../input/d2b/input.txt");
    let commands: Vec<submarine::SubmarineCommand> = contents.lines().map(submarine::SubmarineCommand::from_str)
        .map(|r| r.unwrap()).collect();
    let mut position = submarine::Position::zero();

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
