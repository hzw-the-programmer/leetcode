// 703. Kth Largest Element in a Stream

use core::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut new = Self {
            heap: BinaryHeap::with_capacity(k),
            k,
        };

        for n in nums {
            new.add(n);
        }

        new
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests;
