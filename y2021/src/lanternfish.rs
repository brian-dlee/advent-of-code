use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::ops::{Add, AddAssign};

const SPAWN_INTERVAL: u64 = 7;
const OFFSPRING_DELAY: u64 = 2;

pub struct SpawningModel {
    pub members: HashMap<u64, u64>,
    pub days: usize,
}

impl SpawningModel {
    pub fn new(initial: Vec<u64>) -> SpawningModel {
        let mut model = SpawningModel { members: HashMap::new(), days: 0 };

        for i in 0..(SPAWN_INTERVAL + OFFSPRING_DELAY) {
            model.members.insert(i as u64, 0);
        }

        initial.into_iter().for_each(|f| {
            model.members.get_mut(&f).map(|x| x.add_assign(1));
        });

        model
    }

    pub fn get_total(&self) -> usize {
        self.members.values().fold(0_u64, |result, m| result + *m) as usize
    }

    pub fn print_counts(&self) {
        println!("Printing fish counts");
        for i in 0..(SPAWN_INTERVAL + OFFSPRING_DELAY) {
            println!(" - {} DAY(S) REMAINING = {}", i, self.members[&i]);
        }
    }

    pub fn one_day(&mut self) {
        let mut members: HashMap<u64, u64> = HashMap::new();

        (1..(SPAWN_INTERVAL + OFFSPRING_DELAY)).for_each(|i| {
            members.insert(i - 1, self.members[&i]);
        });

        let reproduced_count = self.members[&0];

        members.insert(
            SPAWN_INTERVAL - 1,
            members.get(&(SPAWN_INTERVAL - 1)).map(|x| *x + reproduced_count).unwrap_or(0)
        );
        members.insert(SPAWN_INTERVAL + OFFSPRING_DELAY - 1, reproduced_count);

        self.members = members;
        self.days.add_assign(1);
    }
}
