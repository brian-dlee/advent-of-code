use std::str::FromStr;
use std::collections::HashSet;
use std::ops::Deref;
use y2021::heightmap::{HeightMap};
use y2021::utils;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub value: u8,
}

impl Point {
    fn move_x(&self, value: i32) -> Point {
        let mut new = self.clone();
        new.x = new.x as i32 + value;
        return new;
    }

    fn move_y(&self, value: i32) -> Point {
        let mut new = self.clone();
        new.y = new.y as i32 + value;
        return new;
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Basin(HashSet<Point>);

impl Basin {
    pub fn get_size(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exploration {
    pub explored: HashSet<Point>,
    pub basin: Basin,
}

impl Exploration {
    pub fn seen(&mut self, point: &Point) {
        self.explored.insert(point.clone());
    }

    pub fn keep(&mut self, point: &Point) {
        self.seen(point);
        self.basin.0.insert(point.clone());
    }

    pub fn previously_visited(&self, point: &Point) -> bool {
        return self.explored.contains(point);
    }
}

pub fn get_risk_level(p: &Point) -> u8 {
    p.value + 1_u8
}

pub fn explore(height_map: &HeightMap, point: &Point, mut exploration: &mut Exploration) {
    println!("Exploring {:?}", point);

    if point.value > 8 {
        println!(" - Marking as seen.");
        exploration.seen(point);
        return;
    }

    println!(" - Marking as basin.");
    exploration.keep(point);

    for (x, y) in [
        (point.x as i32 - 1, point.y as i32),
        (point.x as i32 + 1, point.y as i32),
        (point.x as i32, point.y as i32 - 1),
        (point.x as i32, point.y as i32 + 1),
    ] {
        let point = Point { x, y, value: height_map.get(x, y).unwrap_or(10) };

        if point.value < 9 && !exploration.previously_visited(&point) {
            exploration.keep(&point);
            explore(height_map, &point, exploration);
        } else {
            exploration.seen(&point);
        }
    }
}

pub fn expand(height_map: &HeightMap, low_point: &Point) -> Basin {
    let mut exploration = Exploration {
        explored: HashSet::new(),
        basin: Basin(HashSet::new()),
    };
    explore(height_map, low_point, &mut exploration);
    return exploration.basin.clone();
}

fn scan(height_map: &HeightMap) -> Vec<(i32, i32)> {
    (0..height_map.rows).map(|x| x as i32).flat_map(|row| {
        let init: Vec<(i32, i32)> = Vec::new();
        height_map.hslice(row).into_iter().enumerate().fold(init, |mut result, (col, value)| {
            if height_map.is_low_point(col as i32, row) {
                result.push((col as i32, row));
            }
            result
        })
    }).collect()
}

fn extract_basins(height_map: &HeightMap, low_points: Vec<Point>) -> Vec<Basin> {
    low_points.into_iter().map(|lp| expand(height_map, &lp)).collect()
}

fn main() {
    println!("Starting Day 9a");
    println!("Reading HeightMap");

    let input = HeightMap::from_str(
        utils::read_input("./input/input.txt").trim()
    ).unwrap();

    let low_points: Vec<Point> = scan(&input).into_iter().map(|coord| {
        Point { x: coord.0, y: coord.1, value: input.get(coord.0, coord.1).unwrap() }
    }).collect();

    let mut basins = extract_basins(&input, low_points);

    basins.sort_by(|a, b| a.get_size().partial_cmp(&b.get_size()).unwrap());
    basins.reverse();

    let solution = basins[0..3].iter().map(|b| b.get_size())
        .fold(1_usize, |result, b| result * b);

    println!("Solution: {:?}", solution);
}
