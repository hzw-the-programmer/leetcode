// D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\alloc\src\collections\linked_list.rs

use core::ptr::NonNull;

struct ListNode {
    val: i32,
    next: Option<NonNull<ListNode>>,
    pre: Option<NonNull<ListNode>>,
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
    head: Option<NonNull<ListNode>>,
    tail: Option<NonNull<ListNode>>,
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

        let node = NonNull::from(Box::leak(Box::new(ListNode::new(value))));

        unsafe {
            (*node.as_ptr()).next = self.head;
            (*node.as_ptr()).pre = None;
            let node = Some(node);

            match self.head {
                None => self.tail = node,
                Some(head) => (*head.as_ptr()).pre = node,
            }

            self.head = node;
            self.len += 1;
        }

        true
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let node = NonNull::from(Box::leak(Box::new(ListNode::new(value))));

        unsafe {
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).pre = self.tail;
            let node = Some(node);

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }

        true
    }

    pub fn delete_front(&mut self) -> bool {
        self.head
            .map(|node| unsafe {
                let node = Box::from_raw(node.as_ptr());
                self.head = node.next;

                match self.head {
                    None => self.tail = None,
                    Some(head) => (*head.as_ptr()).pre = None,
                }

                self.len -= 1;
                node
            })
            .is_some()
    }

    pub fn delete_last(&mut self) -> bool {
        self.tail
            .map(|node| unsafe {
                let node = Box::from_raw(node.as_ptr());
                self.tail = node.pre;

                match self.tail {
                    None => self.head = None,
                    Some(tail) => (*tail.as_ptr()).next = None,
                }

                self.len -= 1;
                node
            })
            .is_some()
    }

    pub fn get_front(&self) -> i32 {
        self.head
            .map(|node| unsafe { (*node.as_ptr()).val })
            .unwrap_or(-1)
    }

    pub fn get_rear(&self) -> i32 {
        self.tail
            .map(|node| unsafe { (*node.as_ptr()).val })
            .unwrap_or(-1)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}

impl Drop for MyCircularDeque {
    fn drop(&mut self) {
        while self.delete_front() {}
    }
}
