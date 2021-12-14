use std::str::FromStr;
use y2021::heightmap::{HeightMap};
use y2021::utils;

#[derive(Debug)]
pub struct LowPoint {
    pub x: usize,
    pub y: usize,
    pub value: u8,
}

impl LowPoint {
    pub fn get_risk_level(&self) -> u8 {
        self.value + 1_u8
    }
}

fn scan(height_map: &HeightMap) -> Vec<(usize, usize)> {
    (0..height_map.rows).flat_map(|row| {
        let init: Vec<(usize, usize)> = Vec::new();
        height_map.hslice(row).into_iter().enumerate().fold(init, |mut result, (col, value)| {
            if height_map.is_low_point(col, row) {
                result.push((col, row));
            }
            result
        })
    }).collect()
}

fn main() {
    println!("Starting Day 9a");
    println!("Reading HeightMap");

    let input = HeightMap::from_str(
        utils::read_input("./input/input.txt").trim()
    ).unwrap();

    let low_points: u32 = scan(&input).into_iter().map(|coord| {
        LowPoint { x: coord.0, y: coord.1, value: input.get(coord.0, coord.1).unwrap() }
    }).fold(0, |result, lp| result + lp.get_risk_level() as u32);

    println!("Solution: {:?}", low_points);
}
