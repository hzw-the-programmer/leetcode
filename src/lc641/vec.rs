pub struct MyCircularDeque {
    elements: Vec<i32>,
    rear: usize,
    front: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        Self {
            elements: vec![0; k as usize + 1],
            rear: 0,
            front: 0,
        }
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.front = (self.front + self.elements.len() - 1) % self.elements.len();
            self.elements[self.front] = value;
            true
        }
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.elements[self.rear] = value;
            self.rear = (self.rear + 1) % self.elements.len();
            true
        }
    }

    pub fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.front = (self.front + 1) % self.elements.len();
            true
        }
    }

    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.rear = (self.rear + self.elements.len() - 1) % self.elements.len();
            true
        }
    }

    pub fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.elements[self.front]
        }
    }

    pub fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.elements[(self.rear + self.elements.len() - 1) % self.elements.len()]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rear == self.front
    }

    pub fn is_full(&self) -> bool {
        (self.rear + 1) % self.elements.len() == self.front
    }
}

// pub struct MyCircularDeque {
//     elements: Vec<i32>,
//     rear: usize,
//     len: usize,
// }

// impl MyCircularDeque {
//     pub fn new(k: i32) -> Self {
//         Self {
//             elements: vec![0; k as usize],
//             rear: 0,
//             len: 0,
//         }
//     }

//     pub fn insert_front(&mut self, value: i32) -> bool {
//         if self.is_full() {
//             false
//         } else {
//             let idx = (self.rear + self.elements.len() - self.len - 1) % self.elements.len();
//             self.elements[idx] = value;
//             self.len += 1;
//             true
//         }
//     }

//     pub fn insert_last(&mut self, value: i32) -> bool {
//         if self.is_full() {
//             false
//         } else {
//             self.elements[self.rear] = value;
//             self.rear = (self.rear + 1) % self.elements.len();
//             self.len += 1;
//             true
//         }
//     }

//     pub fn delete_front(&mut self) -> bool {
//         if self.is_empty() {
//             false
//         } else {
//             self.len -= 1;
//             true
//         }
//     }

//     pub fn delete_last(&mut self) -> bool {
//         if self.is_empty() {
//             false
//         } else {
//             self.rear = (self.rear + self.elements.len() - 1) % self.elements.len();
//             self.len -= 1;
//             true
//         }
//     }

//     pub fn get_front(&self) -> i32 {
//         if self.is_empty() {
//             -1
//         } else {
//             self.elements[(self.rear + self.elements.len() - self.len) % self.elements.len()]
//         }
//     }

//     pub fn get_rear(&self) -> i32 {
//         if self.is_empty() {
//             -1
//         } else {
//             self.elements[(self.rear + self.elements.len() - 1) % self.elements.len()]
//         }
//     }

//     pub fn is_empty(&self) -> bool {
//         self.len == 0
//     }

//     pub fn is_full(&self) -> bool {
//         self.len == self.elements.len()
//     }
// }
