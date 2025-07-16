pub struct MinStack {
    data: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            data: vec![],
            min: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.data.push(val);
        match self.min.last() {
            None => self.min.push(val),
            Some(&v) if val <= v => self.min.push(val),
            _ => {}
        }
    }

    pub fn pop(&mut self) {
        self.data.pop().map(|v| {
            if v == *self.min.last().unwrap() {
                self.min.pop();
            }
        });
    }

    pub fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}
