use core::mem;
use std::collections::VecDeque;

pub struct MyStack {
    que1: VecDeque<i32>,
    que2: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self {
            que1: VecDeque::new(),
            que2: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.que2.push_back(x);
        while let Some(n) = self.que1.pop_front() {
            self.que2.push_back(n);
        }
        mem::swap(&mut self.que1, &mut self.que2);
    }

    pub fn pop(&mut self) -> i32 {
        self.que1.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.que1.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.que1.is_empty()
    }
}
