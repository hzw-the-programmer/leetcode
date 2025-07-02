// 622. Design Circular Queue

pub struct MyCircularQueue {
    elements: Vec<i32>,
    rear: usize,
    len: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            elements: vec![0; k as usize],
            rear: 0,
            len: 0,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.elements[self.rear] = value;
            self.rear = (self.rear + 1) % self.elements.len();
            self.len += 1;
            true
        }
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.len -= 1;
            true
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.elements[(self.rear + self.elements.len() - self.len) % self.elements.len()]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.elements[(self.rear + self.elements.len() - 1) % self.elements.len()]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.elements.len()
    }
}

#[cfg(test)]
mod tests;
