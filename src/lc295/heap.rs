use core::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianFinder {
    le: BinaryHeap<i32>,
    ge: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            le: BinaryHeap::new(),
            ge: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        if self.le.is_empty() || num <= *self.le.peek().unwrap() {
            self.le.push(num);
            if self.le.len() > self.ge.len() + 1 {
                self.ge.push(Reverse(self.le.pop().unwrap()));
            }
        } else {
            self.ge.push(Reverse(num));
            if self.ge.len() > self.le.len() {
                self.le.push(self.ge.pop().unwrap().0);
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.le.len() == self.ge.len() {
            self.le
                .peek()
                .zip(self.ge.peek())
                .map(|(&a, &Reverse(b))| (a + b) as f64 / 2 as f64)
                .unwrap()
        } else {
            *self.le.peek().unwrap() as f64
        }
    }
}
