pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            min_stack: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.min_stack.last() {
            None => self.min_stack.push(val),
            Some(v) if val <= *v => self.min_stack.push(val),
            _ => {}
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop().map(|v| {
            if v == *self.min_stack.last().unwrap() {
                self.min_stack.pop();
            }
        });
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}
