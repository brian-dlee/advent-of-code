use std::str::FromStr;
use y2021::utils;
use y2021::submarine;

fn main() {
    println!("Starting Day 2b");

    let contents = utils::read_input("../input/d2b/input.txt");
    let commands: Vec<submarine::SubmarineCommand> = contents.lines().map(submarine::SubmarineCommand::from_str)
        .map(|r| r.unwrap()).collect();
    let mut position = submarine::Position::zero();
    let mut aim = 0;

    println!("Following commands to final position.");

    for command in commands {
        match command.0 {
            submarine::Direction::Forward => {
                position.horizontal += command.1;
                position.depth += command.1 * aim;
            },
            submarine::Direction::Up => aim -= command.1,
            submarine::Direction::Down => aim += command.1,
        }
    }

    println!("Final position: horizontal={}, depth={}", position.horizontal, position.depth);
    println!("Final solution: {}", position.horizontal * position.depth);
}
