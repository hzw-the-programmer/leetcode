// 380. Insert Delete GetRandom O(1)

use rand::prelude::*;
use std::collections::HashMap;

struct RandomizedSet {
    nums: Vec<i32>,
    indices: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            nums: Vec::new(),
            indices: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.indices.contains_key(&val) {
            return false;
        }

        self.indices.insert(val, self.nums.len());
        self.nums.push(val);

        true
    }

    fn remove(&mut self, val: i32) -> bool {
        let Some(&idx) = self.indices.get(&val) else {
            return false;
        };

        let last_idx = self.nums.len() - 1;
        if idx != last_idx {
            self.nums[idx] = self.nums[last_idx];
            self.indices.insert(self.nums[idx], idx);
        }
        self.indices.remove(&val);
        self.nums.pop();

        true
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::rng();
        *self.nums.choose(&mut rng).unwrap()
    }
}

#[cfg(test)]
mod tests;
