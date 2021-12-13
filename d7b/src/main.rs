use std::ops::AddAssign;
use std::str::FromStr;
use y2021::utils;

fn compute_fuel_consumption(index: usize, positions: Vec<u32>) -> u32 {
    positions.into_iter().enumerate().fold(0_u32, |result, (i, value)| {
        result + (1..=((index as i32 - i as i32).abs() as u32)).sum::<u32>() * value
    })
}

fn main() {
    println!("Starting Day 7b");
    println!("Aligning crab submarines");

    let numbers = utils::read_input("./input/sample.txt").trim()
        .split(",").map(|l| l.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut submarines: Vec<u32> = Vec::new();

    numbers.iter().for_each(|n| {
        while submarines.len() as u32 <= *n {
           submarines.push(0);
        }
        submarines.get_mut(*n as usize).map(|x| x.add_assign(1));
    });

    let solution = (0..submarines.len()).into_iter().fold((None, u32::MAX), |result, i| {
        let new_min = std::cmp::min(
            result.1, compute_fuel_consumption(i, submarines.clone())
        );

        return if new_min < result.1 {
            (Some(i), new_min)
        } else {
            result
        }
    });

    println!("Solution: {:?}", solution);
}
