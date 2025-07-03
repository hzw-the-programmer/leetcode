use crate::utils::singly_linked_list::ListNode;
use core::ptr;

pub struct MyCircularQueue {
    head: Option<Box<ListNode>>,
    tail: *mut ListNode,
    capacity: usize,
    len: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            capacity: k as usize,
            len: 0,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            let node = Box::new(ListNode::new(value));
            if self.head.is_none() {
                self.head = Some(node);
                self.tail = self.head.as_deref_mut().unwrap();
            } else {
                unsafe {
                    (*self.tail).next = Some(node);
                    self.tail = (*self.tail).next.as_deref_mut().unwrap();
                }
            }
            self.len += 1;
            true
        }
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            let mut head = self.head.take().unwrap();
            self.head = head.next.take();
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
            true
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.head.as_ref().unwrap().val
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            unsafe { (*self.tail).val }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}
