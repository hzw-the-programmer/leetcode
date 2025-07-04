use std::collections::LinkedList;

pub struct MyHashSet {
    data: Vec<LinkedList<i32>>,
}

impl MyHashSet {
    const BASE: usize = 769;

    pub fn new() -> Self {
        Self {
            data: vec![LinkedList::new(); Self::BASE],
        }
    }

    pub fn add(&mut self, key: i32) {
        let hash = Self::hash(key);
        if !self.data[hash].contains(&key) {
            self.data[hash].push_back(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        let hash = Self::hash(key);
        if let Some(pos) = self.data[hash].iter().position(|&k| k == key) {
            let mut split = self.data[hash].split_off(pos);
            split.pop_front();
            self.data[hash].append(&mut split);
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
