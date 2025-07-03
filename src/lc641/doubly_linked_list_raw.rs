use core::ptr;

struct ListNode {
    val: i32,
    next: *mut ListNode,
    pre: *mut ListNode,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: ptr::null_mut(),
            pre: ptr::null_mut(),
        }
    }
}

pub struct MyCircularDeque {
    head: *mut ListNode,
    tail: *mut ListNode,
    capacity: usize,
    len: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            capacity: k as usize,
            len: 0,
        }
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let raw = Box::into_raw(Box::new(ListNode::new(value)));

        if self.head.is_null() {
            self.head = raw;
            self.tail = raw;
        } else {
            let head = self.head;
            self.head = raw;
            unsafe {
                (*head).pre = raw;
                (*raw).next = head;
            }
        }

        self.len += 1;
        true
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let raw = Box::into_raw(Box::new(ListNode::new(value)));

        if self.head.is_null() {
            self.head = raw;
            self.tail = raw;
        } else {
            let tail = self.tail;
            self.tail = raw;
            unsafe {
                (*raw).pre = tail;
                (*tail).next = raw;
            }
        }

        self.len += 1;
        true
    }

    pub fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        let head = unsafe { Box::from_raw(self.head) };

        if head.next.is_null() {
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else {
            unsafe {
                (*head.next).pre = ptr::null_mut();
            }
            self.head = head.next;
        }

        self.len -= 1;
        true
    }

    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        let tail = unsafe { Box::from_raw(self.tail) };

        if tail.pre.is_null() {
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else {
            unsafe {
                (*tail.pre).next = ptr::null_mut();
            }
            self.tail = tail.pre;
        }

        self.len -= 1;
        true
    }

    pub fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            unsafe { (*self.head).val }
        }
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

impl Drop for MyCircularDeque {
    fn drop(&mut self) {
        while self.delete_front() {}
    }
}
