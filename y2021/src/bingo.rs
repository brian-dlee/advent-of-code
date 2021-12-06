use std::cmp::max;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct BingoResult{
    pub last_number: u32,
    pub remaining_numbers: Vec<usize>,
    pub winners: Vec<Bingo>,
    pub losers: Vec<Bingo>,
}

#[derive(Clone, Debug)]
pub struct Bingo {
    pub id: u32,
    pub rows: usize,
    pub cols: usize,
    pub markers: HashSet<(usize, usize)>,
    data: Vec<Vec<u32>>,
}

impl Bingo {
    pub fn calculate_solution(&self, winning_number: u32) -> u32 {
        let mut sum = 0;

        for x in 0..self.cols {
            for y in 0..self.rows {
                if !self.markers.contains(&(x, y)) {
                    sum += self.get(x, y);
                }
            }
        }

        return sum * winning_number;
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        assert!(y >= 0 && y < self.data.len());
        assert!(x >= 0 && x < self.data[y].len());

        self.data[y][x]
    }

    pub fn is_winning(&self) -> bool {
        for y in 0..self.rows {
            if (0..self.cols).map(|x| self.markers.contains(&(x, y))).all(|o| o) {
                return true;
            }
        }

        for x in 0..self.cols {
            if (0..self.rows).map(|y| self.markers.contains(&(x, y))).all(|o| o) {
                return true;
            }
        }

        return false;
    }

    pub fn mark(&mut self, x: usize, y: usize) {
        assert!(y >= 0 && y < self.data.len());
        assert!(x >= 0 && x < self.data[y].len());

        self.markers.insert((x, y));
    }

    pub fn hslice(&self, row: usize) -> Vec<u32> {
        assert!(row >= 0 && row < self.data.len());

        self.data[row].clone()
    }

    pub fn vslice(&self, column: usize) -> Vec<u32> {
        assert!(column >= 0);

        let mut result = Vec::new();

        for row in &self.data {
            assert!(column < row.len());

            result.push(row[column]);
        }

        result
    }

    pub fn new(input: Vec<String>, id: u32) -> Bingo {
        let mut rows: usize = 0;
        let mut cols: usize = 0;
        let mut data: Vec<Vec<u32>> = Vec::new();

        input.clone().iter().enumerate().for_each(|(i, line)| {
            rows = max(rows, (i + 1));

            line.split_whitespace().enumerate().for_each(|(j, number)| {
                if i >= data.len() {
                    &data.push(Vec::new());
                }

                cols = max(cols, (j + 1));

                &data[i].push(number.parse::<u32>().unwrap());
            })
        });

        Bingo{ id, rows, cols, data, markers: HashSet::new() }
    }
}
