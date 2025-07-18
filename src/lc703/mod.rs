// 703. Kth Largest Element in a Stream

use core::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut res = Self {
            heap: BinaryHeap::new(),
            k: k as usize,
        };

        for n in nums {
            res.add(n);
        }

        res
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else if let Some(&Reverse(n)) = self.heap.peek() {
            if val > n {
                self.heap.pop();
                self.heap.push(Reverse(val));
            }
        }
        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests;
