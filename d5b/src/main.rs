use std::str::FromStr;
use std::num::ParseIntError;
use std::ops::Deref;
use y2021::ocean_vents::{OceanVentMap};
use y2021::utils;

#[derive(PartialEq, Debug)]
enum CoordinateParseError {
    InvalidFormat(ParseIntError),
    IncorrectValueCount(usize),
}

#[derive(Clone, Debug, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl FromStr for Coordinate {
    type Err = CoordinateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(",").collect::<Vec<&str>>() {
            x if x.len() != 2 => Err(CoordinateParseError::IncorrectValueCount(x.len())),
            x => Ok(Coordinate{
                x: x[0].parse::<usize>().map_err(|e| CoordinateParseError::InvalidFormat(e))?,
                y: x[1].parse::<usize>().map_err(|e| CoordinateParseError::InvalidFormat(e))?,
            }),
        }
    }
}

#[derive(PartialEq, Debug)]
enum RayParseError {
    InvalidFormat,
    InvalidCoordinate(CoordinateParseError),
    IncorrectCoordinateCount(usize),
}

#[derive(Clone, Debug)]
struct Ray {
    src: Coordinate,
    dst: Coordinate,
}

impl Ray {
    pub fn is_horizontal(&self) -> bool {
        self.src.y == self.dst.y
    }

    pub fn is_vertical(&self) -> bool {
        self.src.x == self.dst.x
    }

    pub fn points(&self) -> Vec<Coordinate> {
        let mut result: Vec<Coordinate> = Vec::new();
        let mut position = self.src.clone();

        // println!("Finding points from {:?} to {:?}", self.src, self.dst);

        loop {
            // println!(" - [{}] {},{}", result.len(), position.x, position.y);

            result.push(position.clone());

            if position == self.dst {
                break;
            }

            if position.x == self.dst.x {
                if position.y < self.dst.y {
                    position.y += 1;
                } else {
                    position.y -= 1;
                }
            } else {
                if position.x < self.dst.x {
                    position.x += 1;
                } else {
                    position.x -= 1;
                }
            }
        }

        return result;
    }
}

impl FromStr for Ray {
    type Err = RayParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" -> ").collect::<Vec<&str>>() {
            x if x.len() != 2 => Err(RayParseError::IncorrectCoordinateCount(x.len())),
            x => Ok(Ray{
                src: Coordinate::from_str(x[0]).map_err(|e| RayParseError::InvalidCoordinate(e))?,
                dst: Coordinate::from_str(x[1]).map_err(|e| RayParseError::InvalidCoordinate(e))?,
            }),
        }
    }
}

fn parse(lines: Vec<String>) -> Result<Vec<Ray>, RayParseError> {
    lines.into_iter().map(|line| Ray::from_str(line.as_str())).collect()
}

fn main() {
    println!("Starting Day 5a");
    println!("Finding vents.");

    let rays = parse(
        utils::read_input("./input/input.txt").lines().map(|l| l.to_string()).collect::<Vec<String>>()
    ).unwrap();

    let mut vent_map = OceanVentMap::new();

    for ray in rays {
        if !ray.is_horizontal() && !ray.is_vertical() {
            continue;
        }

        for coordinate in ray.points() {
            vent_map.increment(coordinate.x, coordinate.y);
        }
    }

    // println!("{}", vent_map.draw());

    let solution: usize = vent_map.vents.into_iter()
        .filter(|(xy, v)| v > &1_u32).count();

    println!("Number of spots with more than 1 vent: {}", solution);
}
