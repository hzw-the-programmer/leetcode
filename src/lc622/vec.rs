pub struct MyCircularQueue {
    elements: Vec<i32>,
    rear: usize,
    front: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            elements: vec![0; k as usize + 1],
            rear: 0,
            front: 0,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.elements[self.rear] = value;
            self.rear = (self.rear + 1) % self.elements.len();
            true
        }
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.front = (self.front + 1) % self.elements.len();
            true
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.elements[self.front]
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
        self.rear == self.front
    }

    pub fn is_full(&self) -> bool {
        (self.rear + 1) % self.elements.len() == self.front
    }
}

// pub struct MyCircularQueue {
//     elements: Vec<i32>,
//     rear: usize,
//     len: usize,
// }

// impl MyCircularQueue {
//     pub fn new(k: i32) -> Self {
//         Self {
//             elements: vec![0; k as usize],
//             rear: 0,
//             len: 0,
//         }
//     }

//     pub fn en_queue(&mut self, value: i32) -> bool {
//         if self.is_full() {
//             false
//         } else {
//             self.elements[self.rear] = value;
//             self.rear = (self.rear + 1) % self.elements.len();
//             self.len += 1;
//             true
//         }
//     }

//     pub fn de_queue(&mut self) -> bool {
//         if self.is_empty() {
//             false
//         } else {
//             self.len -= 1;
//             true
//         }
//     }

//     pub fn front(&self) -> i32 {
//         if self.is_empty() {
//             -1
//         } else {
//             self.elements[(self.rear + self.elements.len() - self.len) % self.elements.len()]
//         }
//     }

//     pub fn rear(&self) -> i32 {
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

// pub struct MyCircularQueue {
//     elements: Vec<i32>,
//     front: usize,
//     back: usize,
//     len: usize,
// }

// impl MyCircularQueue {
//     pub fn new(k: i32) -> Self {
//         Self {
//             elements: vec![0; k as usize],
//             front: 0,
//             back: 0,
//             len: 0,
//         }
//     }

//     pub fn en_queue(&mut self, value: i32) -> bool {
//         if self.is_full() {
//             false
//         } else {
//             self.elements[self.back] = value;
//             self.back = (self.back + 1) % self.elements.len();
//             self.len += 1;
//             true
//         }
//     }

//     pub fn de_queue(&mut self) -> bool {
//         if self.is_empty() {
//             false
//         } else {
//             self.front = (self.front + 1) % self.elements.len();
//             self.len -= 1;
//             true
//         }
//     }

//     pub fn front(&self) -> i32 {
//         if self.is_empty() {
//             -1
//         } else {
//             self.elements[self.front]
//         }
//     }

//     pub fn rear(&self) -> i32 {
//         if self.is_empty() {
//             -1
//         } else {
//             let idx = if self.back == 0 {
//                 self.elements.len() - 1
//             } else {
//                 self.back - 1
//             };
//             self.elements[idx]
//         }
//     }

//     pub fn is_empty(&self) -> bool {
//         self.len == 0
//     }

//     pub fn is_full(&self) -> bool {
//         self.len == self.elements.len()
//     }
// }
