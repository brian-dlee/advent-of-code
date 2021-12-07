use std::collections::hash_map::HashMap;
use std::num::ParseIntError;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum CoordinateParseError {
    InvalidFormat(ParseIntError),
    IncorrectValueCount(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
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
pub enum RayParseError {
    InvalidFormat,
    InvalidCoordinate(CoordinateParseError),
    IncorrectCoordinateCount(usize),
}

#[derive(Clone, Debug)]
pub struct Ray {
    pub src: Coordinate,
    pub dst: Coordinate,
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

            if position.y != self.dst.y {
                if position.y < self.dst.y {
                    position.y += 1;
                } else {
                    position.y -= 1;
                }
            }

            if position.x != self.dst.x {
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

pub struct OceanVentMap {
    pub rows: usize,
    pub cols: usize,
    pub vents: HashMap<(usize, usize), u32>,
}

impl OceanVentMap {
    pub fn new() -> OceanVentMap {
        OceanVentMap {
            rows: 0,
            cols: 0,
            vents: HashMap::new(),
        }
    }

    pub fn increment(&mut self, x: usize, y: usize) {
        if x >= self.cols {
            self.cols = x + 1;
        }

        if y >= self.rows {
            self.rows = y + 1;
        }

        match self.vents.get_mut(&(x, y)) {
            Some(n) => {
                n.add_assign(1);
            },
            None => {
                self.vents.insert((x, y), 1);
            },
        }
    }

    pub fn draw(&self) -> String {
        let mut output = String::new();

        for y in 0..self.rows {
            for x in 0..self.cols {
                output.extend(match self.vents.get(&(x, y)) {
                    Some(v) => v.to_string(),
                    None => ".".to_string()
                }.chars());
            }

            output.push('\n');
        }

        return output;
    }
}
