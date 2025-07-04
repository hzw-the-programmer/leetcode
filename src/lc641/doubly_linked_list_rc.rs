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
            return false;
        }

        let node = Rc::new(RefCell::new(ListNode::new(value)));

        match self.head.take() {
            None => self.tail = Some(node.clone()),
            Some(head) => {
                head.borrow_mut().pre = Some(Rc::downgrade(&node));
                node.borrow_mut().next = Some(head);
            }
        }

        self.head = Some(node);
        self.len += 1;

        true
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let node = Rc::new(RefCell::new(ListNode::new(value)));

        match self.tail.take() {
            None => self.head = Some(node.clone()),
            Some(tail) => {
                node.borrow_mut().pre = Some(Rc::downgrade(&tail));
                tail.borrow_mut().next = Some(node.clone());
            }
        }

        self.tail = Some(node);
        self.len += 1;

        true
    }

    pub fn delete_front(&mut self) -> bool {
        self.head
            .take()
            .map(|node| {
                self.head = node.borrow_mut().next.take();

                match self.head.as_mut() {
                    None => self.tail = None,
                    Some(head) => head.borrow_mut().pre = None,
                }

                self.len -= 1;
                node
            })
            .is_some()
    }

    pub fn delete_last(&mut self) -> bool {
        self.tail
            .take()
            .map(|node| {
                match node.borrow_mut().pre.take() {
                    None => self.head = None,
                    Some(pre) => {
                        let pre = pre.upgrade().unwrap();
                        pre.borrow_mut().next = None;
                        self.tail = Some(pre);
                    }
                }

                self.len -= 1;
                node
            })
            .is_some()
    }

    pub fn get_front(&self) -> i32 {
        self.head
            .as_ref()
            .map(|node| node.borrow().val)
            .unwrap_or(-1)
    }

    pub fn get_rear(&self) -> i32 {
        self.tail
            .as_ref()
            .map(|node| node.borrow().val)
            .unwrap_or(-1)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}
