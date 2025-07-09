use core::ptr::NonNull;
use std::collections::HashMap;

use super::{Iter, LinkedList, Node};

pub struct LRUCache {
    map: HashMap<i32, NonNull<Node<(i32, i32)>>>,
    list: LinkedList<(i32, i32)>,
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            map: HashMap::with_capacity(capacity + 1),
            list: LinkedList::new(),
            capacity: capacity,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.map.get(&key).map_or(-1, |&node| {
            self.list.move_to_head(node);
            unsafe { (*node.as_ptr()).val.1 }
        })
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.map
            .entry(key)
            .and_modify(|&mut node| {
                unsafe { (*node.as_ptr()).val.1 = value };
                self.list.move_to_head(node);
            })
            .or_insert_with(|| {
                self.list.push_front((key, value));
                self.list.peek_front_node().unwrap()
            });

        if self.list.len() > self.capacity {
            if let Some(val) = self.list.pop_back() {
                self.map.remove(&val.0);
            }
        }
    }

    pub fn iter(&self) -> Iter<(i32, i32)> {
        self.list.iter()
    }
}
