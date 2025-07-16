// 155. Min Stack

struct MinStack {
    data: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            data: vec![],
            min: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        match self.min.last() {
            None => self.min.push(val),
            Some(&v) if val <= v => self.min.push(val),
            _ => {}
        }
    }

    fn pop(&mut self) {
        self.data.pop().map(|v| {
            if v == *self.min.last().unwrap() {
                self.min.pop();
            }
        });
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
mod tests;
