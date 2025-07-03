use core::cell::RefCell;
use std::rc::{Rc, Weak};

struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
    pre: Option<Weak<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            pre: None,
        }
    }
}

pub struct MyCircularDeque {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
    capacity: usize,
    len: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        Self {
            head: None,
            tail: None,
            capacity: k as usize,
            len: 0,
        }
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            let node = Rc::new(RefCell::new(ListNode::new(value)));
            if self.head.is_none() {
                self.head = Some(node);
                self.tail = self.head.clone();
            } else {
                let head = self.head.take().unwrap();
                head.borrow_mut().pre = Some(Rc::downgrade(&node));
                node.borrow_mut().next = Some(head);
                self.head = Some(node);
            }
            self.len += 1;
            true
        }
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            let node = Rc::new(RefCell::new(ListNode::new(value)));
            if self.head.is_none() {
                self.head = Some(node);
                self.tail = self.head.clone();
            } else {
                let tail = self.tail.take().unwrap();
                node.borrow_mut().pre = Some(Rc::downgrade(&tail));
                tail.borrow_mut().next = Some(node.clone());
                self.tail = Some(node);
            }
            self.len += 1;
            true
        }
    }

    pub fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            let head = self.head.take().unwrap();
            if let Some(node) = head.borrow_mut().next.take() {
                node.borrow_mut().pre = None;
                self.head = Some(node);
            } else {
                self.tail = None;
            }
            self.len -= 1;
            true
        }
    }

    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            let tail = self.tail.take().unwrap();
            if let Some(node) = tail.borrow_mut().pre.take() {
                let node = node.upgrade().unwrap();
                node.borrow_mut().next = None;
                self.tail = Some(node);
            } else {
                self.head = None;
            }
            self.len -= 1;
            true
        }
    }

    pub fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.head.as_ref().unwrap().borrow().val
        }
    }

    pub fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.tail.as_ref().unwrap().borrow().val
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}
