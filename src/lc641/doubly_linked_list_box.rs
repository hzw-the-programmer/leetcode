use core::ptr;

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
    pre: *mut ListNode,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            pre: ptr::null_mut(),
        }
    }
}

pub struct MyCircularDeque {
    head: Option<Box<ListNode>>,
    tail: *mut ListNode,
    capacity: usize,
    len: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            capacity: k as usize,
            len: 0,
        }
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let mut node = Box::new(ListNode::new(value));

        match self.head.take() {
            None => self.tail = &mut *node,
            Some(mut head) => {
                head.pre = &mut *node;
                node.next = Some(head);
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

        let mut node = Box::new(ListNode::new(value));

        if self.head.is_none() {
            self.tail = &mut *node;
            self.head = Some(node);
        } else {
            let tail = self.tail;
            self.tail = &mut *node;
            node.pre = tail;
            unsafe { (*tail).next = Some(node) };
        }

        self.len += 1;
        true
    }

    pub fn delete_front(&mut self) -> bool {
        self.head
            .take()
            .map(|mut node| {
                self.head = node.next.take();

                match self.head.as_mut() {
                    None => self.tail = ptr::null_mut(),
                    Some(head) => head.pre = ptr::null_mut(),
                }

                self.len -= 1;
                node
            })
            .is_some()
    }

    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            if unsafe { !(*self.tail).pre.is_null() } {
                unsafe {
                    (*(*self.tail).pre).next = None;
                    self.tail = (*self.tail).pre;
                }
            } else {
                self.head = None;
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
            true
        }
    }

    pub fn get_front(&self) -> i32 {
        self.head.as_ref().map(|node| node.val).unwrap_or(-1)
    }

    pub fn get_rear(&self) -> i32 {
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
