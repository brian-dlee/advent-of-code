use std::cmp::max;
use std::collections::hash_map::Keys;
use std::collections::hash_set::Iter;
use std::str::FromStr;
use std::collections::HashSet;
use std::iter::Map;
use std::num::ParseIntError;
use y2021::bingo::{Bingo, BingoResult};
use y2021::utils;
use y2021::submarine;
use crate::RayParseError::InvalidFormat;

#[derive(PartialEq, Debug)]
enum CoordinateParseError {
    InvalidFormat(ParseIntError),
    IncorrectValueCount(usize),
}

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

struct Ray {
    src: Coordinate,
    dst: Coordinate,
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

    let pairs = parse(
        utils::read_input("./input/sample.txt").lines().map(|l| l.to_string()).collect::<Vec<String>>()
    ).unwrap();
}
