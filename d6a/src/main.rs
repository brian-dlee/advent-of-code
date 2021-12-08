use std::str::FromStr;
use y2021::lanternfish::{SpawningModel};
use y2021::utils;

fn main() {
    println!("Starting Day 6a");
    println!("Modeling lanternfish growth.");

    let numbers = utils::read_input("./input/input.txt").trim()
        .split(",").map(|l| l.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let days_to_simulate = 80;

    let mut model = SpawningModel::new(numbers);

    for i in 0..days_to_simulate {
        model.one_day();
    }

    println!("Number of fish after {} days: {}", days_to_simulate, model.get_total());
}
