use core::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Copy, Clone)]
struct Node {
    key: i32,
    val: i32,
    freq: usize,
    time: usize,
}

impl Node {
    fn new(key: i32, val: i32, freq: usize, time: usize) -> Self {
        Self {
            key,
            val,
            freq,
            time,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.time == other.time
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.freq.partial_cmp(&other.freq) {
            Some(Ordering::Equal) => self.time.partial_cmp(&other.time),
            res => res,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq.cmp(&other.freq) {
            Ordering::Equal => self.time.cmp(&other.time),
            res => res,
        }
    }
}

pub struct LFUCache {
    map: HashMap<i32, Node>,
    set: BTreeSet<Node>,
    capacity: usize,
    time: usize,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            map: HashMap::with_capacity(capacity),
            set: BTreeSet::new(),
            capacity,
            time: 0,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.map.get_mut(&key).map_or(-1, |node| {
            self.set.remove(node);

            node.freq += 1;

            self.time += 1;
            node.time = self.time;

            self.set.insert(*node);
            node.val
        })
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get_mut(&key) {
            node.val = value;
            self.get(key);
            return;
        }

        if self.map.len() == self.capacity {
            let node = self.set.pop_first().unwrap();
            self.map.remove(&node.key);
        }

        self.time += 1;
        let node = Node::new(key, value, 1, self.time);

        self.map.insert(key, node);
        self.set.insert(node);
    }
}
