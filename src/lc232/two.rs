pub struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stack2.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.swap();
        self.stack1.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        self.swap();
        *self.stack1.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }

    fn swap(&mut self) {
        if self.stack1.is_empty() {
            while let Some(n) = self.stack2.pop() {
                self.stack1.push(n);
            }
        }
    }
}
