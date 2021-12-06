use std::cmp::max;
use std::collections::hash_map::Keys;
use std::collections::hash_set::Iter;
use std::str::FromStr;
use std::collections::HashSet;
use std::iter::Map;
use y2021::bingo::{Bingo, BingoResult};
use y2021::utils;
use y2021::submarine;

fn parse(lines: Vec<String>) -> (Vec<usize>, Vec<Bingo>) {
    return (
        lines[0].split(',').map(|c| c.parse::<usize>().unwrap()).collect(),
        lines[1..lines.len()].iter().fold(Vec::new(), |mut result, line| {
            let size = result.len();

            if line.trim().len() == 0 {
                result.push(Vec::new());
            } else {
                result[size - 1].push(line.clone());
            }

            return result;
        }).iter().enumerate().map(
            |(id, group)| Bingo::new(group.clone(), id as u32)
        ).collect(),
    )
}

fn play(numbers: Vec<usize>, mut cards: Vec<Bingo>) -> BingoResult {
    let total_markers: HashSet<(usize, usize)> = cards.iter().fold(HashSet::new(), |result, card| {
        card.markers.union(&result).map(|x| *x).collect()
    });

    println!("Starting a new game with {} cards. {} numbers to call. {} spaces are marked", cards.len(), numbers.len(), total_markers.len());

    let mut call: usize = 0;
    let mut number: usize = 0;
    let mut results: (Vec<Bingo>, Vec<Bingo>) = (Vec::new(), cards.clone());

    while call < numbers.len() && results.0.is_empty() {
        number = numbers[call];

        println!(" - Calling number {}", number);

        for mut card in cards.iter_mut() {
            for x in 0..card.rows {
                for y in 0..card.cols {
                    if card.get(x, y) == number as u32 {
                        card.mark(x as usize, y as usize);
                    }
                }
            }
        }

        results = cards.iter().map(|x| x.clone()).partition(|x| x.is_winning());
        call += 1;
    }

    BingoResult{
        last_number: number as u32,
        winners: results.0,
        remaining_numbers: numbers[call..numbers.len()].to_vec(),
        losers: results.1,
    }
}

fn main() {
    println!("Starting Day 4a");
    println!("Playing Bingo.");

    let mut last_result: Option<BingoResult> = None;
    let (mut numbers, mut cards) = parse(
        utils::read_input("./input/input.txt").lines().map(|l| l.to_string()).collect::<Vec<String>>()
    );

    while cards.len() > 0 {
        let result = play(numbers.clone(), cards.clone());

        last_result = Some(result.clone());
        cards = result.losers.clone();
        numbers = result.remaining_numbers.clone();

        if result.winners.is_empty() {
            break;
        }
    }

    match last_result {
        Some(result) => {
            match result.winners {
                winners if winners.len() == 1 => {
                    println!("Last winners; WinningNumber={}, Solution={:?}", result.last_number, winners[0].calculate_solution(result.last_number));
                },
                _ => {
                    println!("No winners in last result.")
                }
            }
        },
        None => {
            println!("No last result. Were no games played?")
        }
    }
}
