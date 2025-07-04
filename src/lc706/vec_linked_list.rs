use std::collections::LinkedList;

pub struct MyHashMap {
    buckets: Vec<LinkedList<(i32, i32)>>,
}

impl MyHashMap {
    const BASE: usize = 769;

    pub fn new() -> Self {
        Self {
            buckets: vec![LinkedList::new(); Self::BASE],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let hash = Self::hash(key);
        // if let Some(p) = self.buckets[hash].iter_mut().find(|(k, _)| *k == key) {
        if let Some(p) = self.buckets[hash].iter_mut().find(|&&mut (k, _)| k == key) {
            p.1 = value;
        } else {
            self.buckets[hash].push_back((key, value));
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        let hash = Self::hash(key);
        self.buckets[hash]
            .iter()
            // .find(|(k, _)| *k == key)
            .find(|&&(k, _)| k == key)
            // .map(|(_, v)| *v)
            // .map(|&(_, v)| v)
            // .unwrap_or(-1)
            .map_or(-1, |&(_, v)| v)
    }

    pub fn remove(&mut self, key: i32) {
        let hash = Self::hash(key);
        if let Some(pos) = self.buckets[hash].iter().position(|&(k, _)| k == key) {
            let mut split = self.buckets[hash].split_off(pos);
            split.pop_front();
            self.buckets[hash].append(&mut split);
        }
    }

    fn hash(key: i32) -> usize {
        key as usize % Self::BASE
    }
}
