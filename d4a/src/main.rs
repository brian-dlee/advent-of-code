use std::cmp::max;
use std::collections::hash_map::Keys;
use std::collections::hash_set::Iter;
use std::str::FromStr;
use std::collections::HashSet;
use std::iter::Map;
use y2021::grid::{Grid, Selection};
use y2021::utils;
use y2021::submarine;

fn parse(lines: Vec<String>) -> (Vec<u32>, Vec<Grid>) {
    return (
        lines[0].split(',').map(|c| c.parse::<u32>().unwrap()).collect(),
        lines[1..lines.len()].iter().fold(Vec::new(), |mut result, line| {
            if line.trim().len() == 0 {
                result.push(Vec::new());
            } else {
                result[result.len() - 1].push(line.clone());
            }

            return result;
        }).map(|group| Grid::new(group)),
    )
}

fn main() {
    println!("Starting Day 4a");
    println!("Playing Bingo.");

    let (numbers, grids) = parse(
        utils::read_input("./input/input.txt").lines().map(|l| l.to_string()).collect::<Vec<String>>()
    );
    let called: HashSet<u32> = HashSet::new();
    let grid_markers: Vec<Vec<bool>> = grids.map(|grid| {

    });

    for number in numbers {
        for grid in grids {
            for x in 0..grid.rows {
                for y in 0..grid.cols {
                    if grid.get(x, y) == number {

                    }
                }
            }
        }
    }

    println!("Counts: OxyGen={:?}, CO2Scrub={:?}", result.0, result.1);
    println!("Final solution: {}", result.0 * result.1);
}
