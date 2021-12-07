use std::collections::hash_map::HashMap;
use std::ops::AddAssign;

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
