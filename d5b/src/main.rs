use std::str::FromStr;
use std::ops::Deref;
use y2021::ocean_vents::{OceanVentMap, Coordinate, Ray, RayParseError};
use y2021::utils;

fn parse(lines: Vec<String>) -> Result<Vec<Ray>, RayParseError> {
    lines.into_iter().map(|line| Ray::from_str(line.as_str())).collect()
}

fn main() {
    println!("Starting Day 5b");
    println!("Finding vents (allowing diagonals).");

    let rays = parse(
        utils::read_input("./input/input.txt").lines().map(|l| l.to_string()).collect::<Vec<String>>()
    ).unwrap();

    let mut vent_map = OceanVentMap::new();

    for ray in rays {
        for coordinate in ray.points() {
            vent_map.increment(coordinate.x, coordinate.y);
        }
    }

    // println!("{}", vent_map.draw());

    let solution: usize = vent_map.vents.into_iter()
        .filter(|(xy, v)| v > &1_u32).count();

    println!("Number of spots with more than 1 vent: {}", solution);
}
