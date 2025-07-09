use crate::lc146::{LinkedList, Node};
use core::ptr::NonNull;
use std::collections::HashMap;

#[derive(Default, Debug)]
struct Item {
    key: i32,
    val: i32,
    freq: usize,
}

pub struct LFUCache {
    key_map: HashMap<i32, NonNull<Node<Item>>>,
    freq_map: HashMap<usize, LinkedList<Item>>,
    min_freq: usize,
    capacity: usize,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            key_map: HashMap::with_capacity(capacity as usize),
            freq_map: HashMap::new(),
            min_freq: 0,
            capacity,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.key_map.get(&key).map_or(-1, |&node| unsafe {
            let old_freq = (*node.as_ptr()).val.freq;
            let new_freq = old_freq + 1;

            self.freq_map.get_mut(&old_freq).unwrap().unlink(node);
            if self.freq_map[&old_freq].is_empty() {
                self.freq_map.remove(&old_freq);
                if old_freq == self.min_freq {
                    self.min_freq = new_freq;
                }
            }

            (*node.as_ptr()).val.freq = new_freq;
            self.freq_map
                .entry(new_freq)
                .or_insert_with(|| LinkedList::new())
                .link_after_head(node);

            (*node.as_ptr()).val.val
        })
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.key_map.get_mut(&key) {
            unsafe { (*node.as_ptr()).val.val = value };
            self.get(key);
            return;
        }

        if self.key_map.len() == self.capacity {
            let item = self
                .freq_map
                .get_mut(&self.min_freq)
                .unwrap()
                .pop_back()
                .unwrap();
            if self.freq_map[&self.min_freq].is_empty() {
                self.freq_map.remove(&self.min_freq);
            }
            self.key_map.remove(&item.key);
        }

        let list = self.freq_map.entry(1).or_insert_with(|| LinkedList::new());
        list.push_front(Item {
            key,
            val: value,
            freq: 1,
        });
        self.key_map.insert(key, list.peek_front_node().unwrap());
        self.min_freq = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::LFUCache;

    impl LFUCache {
        pub fn dump(&self) {
            println!("");
            for (k, v) in &self.freq_map {
                println!("{k} {v:?}");
            }
            println!("");
        }
    }
}
