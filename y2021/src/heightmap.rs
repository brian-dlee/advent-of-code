use std::cmp::max;
use std::num::ParseIntError;
use std::str::FromStr;

const MAX: u8 = 10;

#[derive(Debug)]
pub enum ParseHeightMapError {
    InvalidDigit(ParseIntError),
}

#[derive(Clone, Debug)]
pub struct HeightMap {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Vec<u8>>,
}

impl HeightMap {
    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        match x {
            x if x < 0 => None,
            x if x >= self.cols => None,
            x => match y {
                y if y < 0 => None,
                y if y >= self.rows => None,
                y => Some(self.data[y][x].clone())
            }
        }
    }

    fn get_col_adjacent(&self, col: usize, row: usize, modifier: i8) -> u8 {
        if col == 0 && modifier < 0 {
            None
        } else {
            self.get((col as i8 + modifier) as usize, row)
        }.unwrap_or(MAX)
    }

    fn get_row_adjacent(&self, col: usize, row: usize, modifier: i8) -> u8 {
        if row == 0 && modifier < 0 {
            None
        } else {
            self.get(col, (row as i8 + modifier) as usize)
        }.unwrap_or(MAX)
    }

    pub fn is_low_point(&self, col: usize, row: usize) -> bool {
        let value = self.get(col, row).unwrap();
        [
            self.get_col_adjacent(col, row, -1),
            self.get_col_adjacent(col, row, 1),
            self.get_row_adjacent(col, row, -1),
            self.get_row_adjacent(col, row, 1),
        ].into_iter().fold(true, |result, other| {
            result && value < other
        })
    }

    pub fn hslice(&self, row: usize) -> Vec<u8> {
        assert!(row >= 0 && row < self.data.len());

        self.data[row].clone()
    }

    pub fn vslice(&self, column: usize) -> Vec<u8> {
        assert!(column >= 0);

        let mut result = Vec::new();

        for row in &self.data {
            assert!(column < row.len());

            result.push(row[column]);
        }

        result
    }
}

impl FromStr for HeightMap {
    type Err = ParseHeightMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: usize = 0;
        let mut cols: usize = 0;

        s.lines().enumerate().map(|(i, line)| {
            rows = max(rows, (i + 1));

            line.chars().enumerate().map(|(j, c)| {
                cols = max(cols, (j + 1));

                c.to_string().parse::<u8>().map_err(|e| ParseHeightMapError::InvalidDigit(e))
            }).collect::<Result<Vec<u8>, ParseHeightMapError>>()
        }).collect::<Result<Vec<Vec<u8>>, ParseHeightMapError>>().map(|data| {
           HeightMap { data, rows, cols }
        })
    }
}
