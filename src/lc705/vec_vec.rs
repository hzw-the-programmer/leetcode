pub struct MyHashSet {
    data: Vec<Vec<i32>>,
}

impl MyHashSet {
    const BASE: usize = 769;

    pub fn new() -> Self {
        Self {
            data: vec![vec![]; Self::BASE],
        }
    }

    pub fn add(&mut self, key: i32) {
        let hash = Self::hash(key);
        if !self.data[hash].contains(&key) {
            self.data[hash].push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        let hash = Self::hash(key);
        if let Some(pos) = self.data[hash].iter().position(|&k| k == key) {
            self.data[hash].swap_remove(pos);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let hash = Self::hash(key);
        self.data[hash].contains(&key)
    }

    fn hash(key: i32) -> usize {
        key as usize % Self::BASE
    }
}
