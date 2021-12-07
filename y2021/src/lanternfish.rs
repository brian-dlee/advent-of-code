use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::ops::{Add, AddAssign};

const SPAWN_INTERVAL: u32 = 7;
const OFFSPRING_DELAY: u32 = 2;

pub struct SpawningModel {
    pub members: HashMap<u32, HashSet<u32>>,
    pub days: usize,
    next_id: u32,
}

impl SpawningModel {
    pub fn new(initial: Vec<u32>) -> SpawningModel {
        let mut model = SpawningModel { members: HashMap::new(), days: 0, next_id: 0 };

        for i in 0..(SPAWN_INTERVAL + OFFSPRING_DELAY) {
            model.members.insert(i as u32, HashSet::new());
        }

        initial.into_iter().for_each(|f| {
            model.members.get_mut(&f).unwrap().insert(model.next_id);
            model.next_id.add_assign(1);
        });

        model
    }

    pub fn get_total(&self) -> usize {
        return self.next_id as usize;
    }

    pub fn one_day(&mut self) {
        println!("Simulating Day {}", self.days + 1);

        let mut members: HashMap<u32, HashSet<u32>> = HashMap::new();

        for i in 0..(SPAWN_INTERVAL + OFFSPRING_DELAY) {
            members.insert(i as u32, HashSet::new());
        }

        (0..(SPAWN_INTERVAL + OFFSPRING_DELAY)).into_iter().flat_map(|i| {
            self.members[&(i as u32)].clone().into_iter().map(|id| (i, id)).collect::<Vec<(u32, u32)>>()
        }).for_each(|(days_remaining, id)| {
            if days_remaining == 0 {
                println!(" - Fish {} spawned offspring {}. Fish {} at {}; Fish {} at {}.", id, self.next_id, id, SPAWN_INTERVAL - 1, self.next_id, SPAWN_INTERVAL + OFFSPRING_DELAY - 1);
                members.get_mut(&(SPAWN_INTERVAL + OFFSPRING_DELAY - 1)).unwrap().insert(self.next_id);
                members.get_mut(&(SPAWN_INTERVAL - 1)).unwrap().insert(id);
                self.next_id.add_assign(1);
            } else {
                println!(" - Fish {} at {}.", id, days_remaining - 1);
                members.get_mut(&(days_remaining - 1)).unwrap().insert(id);
            }
        });

        self.members = members;
        self.days.add_assign(1);
    }
}
