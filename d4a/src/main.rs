use std::cmp::max;
use std::collections::hash_map::Keys;
use std::collections::hash_set::Iter;
use std::str::FromStr;
use std::collections::HashSet;
use std::iter::Map;
use y2021::bingo::Bingo;
use y2021::utils;
use y2021::submarine;

fn parse(lines: Vec<String>) -> (Vec<usize>, Vec<Bingo>) {
    return (
        lines[0].split(',').map(|c| c.parse::<usize>().unwrap()).collect(),
        lines[1..lines.len()].iter().fold(Vec::new(), |mut result, line| {
            let size = result.len();

            println!("Line {}, Size {}", line, size);

            if line.trim().len() == 0 {
                result.push(Vec::new());
            } else {
                result[size - 1].push(line.clone());
            }

            return result;
        }).iter().map(|group| Bingo::new(group.clone())).collect(),
    )
}

fn main() {
    println!("Starting Day 4a");
    println!("Playing Bingo.");

    let (numbers, mut cards) = parse(
        utils::read_input("./input/input.txt").lines().map(|l| l.to_string()).collect::<Vec<String>>()
    );

    let mut call = 0;
    let mut winner: Option<(u32, Bingo)> = None;

    while call < numbers.len() && winner.is_none() {
        let number = numbers[call];

        for mut card in cards.iter_mut() {
            for x in 0..card.rows {
                for y in 0..card.cols {
                    if card.get(x, y) == number as u32 {
                        card.mark(x as usize, y as usize);
                    }
                }
            }
        }

        match utils::first(cards.iter().flat_map(|x| if x.is_winning() { Some(x.clone()) } else { None }).collect::<Vec<Bingo>>()) {
            Some(x) => { winner = Some((number as u32, x)); }
            _ => (),
        }

        call += 1;
    }

    match winner {
        Some((number, card)) => {
            println!("Winner; WinningNumber={}, Solution={}", number, card.calculate_solution(number));
        },
        None => println!("No winner"),
    }

}
