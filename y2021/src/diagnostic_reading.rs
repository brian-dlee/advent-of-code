use std::cmp::max;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct DiagnosticReading {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Vec<char>>,
}

pub struct Selection {
    pub keep: HashSet<usize>,
    pub remove: HashSet<usize>,
}

impl DiagnosticReading {
    pub fn get(&self, x: usize, y: usize) -> char {
        assert!(y >= 0 && y < self.data.len());
        assert!(x >= 0 && x < self.data[y].len());

        self.data[y][x]
    }

    pub fn without_rows<'a, I>(&mut self, rows: I) -> DiagnosticReading where I: Iterator<Item=&'a usize> {
        let removals: HashSet<&usize> = HashSet::from_iter(rows);
        let data = self.data.iter().enumerate()
            .filter(|(i, r)| !removals.contains(i))
            .map(|(i, r)| r.clone()).collect::<Vec<Vec<char>>>();

        DiagnosticReading {
            cols: self.cols,
            rows: data.len(),
            data,
        }
    }

    pub fn hslice(&self, row: usize) -> Vec<char> {
        assert!(row >= 0 && row < self.data.len());

        self.data[row].clone()
    }

    pub fn vslice(&self, column: usize) -> Vec<char> {
        assert!(column >= 0);

        let mut result = Vec::new();

        for row in &self.data {
            assert!(column < row.len());

            result.push(row[column]);
        }

        result
    }

    pub fn new(input: Vec<String>) -> DiagnosticReading {
        let mut rows: usize = 0;
        let mut cols: usize = 0;
        let mut data: Vec<Vec<char>> = Vec::new();

        input.clone().iter().enumerate().for_each(|(i, line)| {
            rows = max(rows, (i + 1));

            line.chars().enumerate().for_each(|(j, char)| {
                if i >= data.len() {
                    &data.push(Vec::new());
                }

                cols = max(cols, (j + 1));

                &data[i].push(char);
            })
        });

        DiagnosticReading{ rows, cols, data, }
    }
}
