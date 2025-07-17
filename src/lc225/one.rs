use std::collections::VecDeque;

pub struct MyStack {
    que: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self {
            que: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        let mut len = self.que.len();
        self.que.push_back(x);
        while len > 0 {
            let n = self.que.pop_front().unwrap();
            self.que.push_back(n);
            len -= 1;
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.que.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.que.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.que.is_empty()
    }
}
