use core::ptr::NonNull;

mod into_iter;
mod iter;
mod iter_mut;

use iter_mut::IterMut;

pub struct MyLinkedList {
    head: NonNull<Node>,
    tail: NonNull<Node>,
    len: usize,
}

impl MyLinkedList {
    pub fn new() -> Self {
        let head = NonNull::from(Box::leak(Box::new(Node::new(0))));
        let tail = NonNull::from(Box::leak(Box::new(Node::new(0))));

        unsafe {
            (*head.as_ptr()).next = Some(tail);
            (*tail.as_ptr()).prev = Some(head);
        }

        Self { head, tail, len: 0 }
    }

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.len as i32 {
            return -1;
        }

        let index = index as usize;

        if index + 1 < self.len - index {
            // println!("{index}: nth");
            self.iter().nth(index).map_or(-1, |&n| n)
        } else {
            // println!("{index}: nth_back");
            let index = self.len - 1 - index;
            self.iter().nth_back(index).map_or(-1, |&n| n)
        }
    }

    pub fn add_at_head(&mut self, val: i32) {
        self.push_front(val);
    }

    pub fn add_at_tail(&mut self, val: i32) {
        self.push_back(val);
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        let index = index as usize;
        if let Some(mut predecessor) = self.predecessor_mut(index) {
            let mut node = NonNull::from(Box::leak(Box::new(Node::new(val))));

            unsafe {
                let mut old = predecessor.as_ref().next.unwrap();
                predecessor.as_mut().next = Some(node);
                node.as_mut().next = Some(old);
                old.as_mut().prev = Some(node);
                node.as_mut().prev = Some(predecessor);

                self.len += 1;
            }
        } else if index == self.len {
            self.push_back(val);
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        let index = index as usize;
        if let Some(mut predecessor) = self.predecessor_mut(index) {
            unsafe {
                let old = predecessor.as_ref().next.unwrap();
                let mut new = old.as_ref().next.unwrap();
                predecessor.as_mut().next = Some(new);
                new.as_mut().prev = Some(predecessor);

                self.len -= 1;

                let _ = Box::from_raw(old.as_ptr());
            }
        }
    }

    pub fn push_front(&mut self, val: i32) {
        let mut node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            let mut old = self.head.as_ref().next.unwrap();
            self.head.as_mut().next = Some(node);
            node.as_mut().next = Some(old);
            node.as_mut().prev = old.as_ref().prev;
            old.as_mut().prev = Some(node);

            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        unsafe {
            // if self.len == 0 {
            if self.head.as_ref().next == Some(self.tail) {
                None
            } else {
                let old = self.head.as_ref().next.unwrap();
                let mut new = old.as_ref().next.unwrap();
                self.head.as_mut().next = Some(new);
                new.as_mut().prev = old.as_ref().prev;

                self.len -= 1;

                let node = Box::from_raw(old.as_ptr());
                Some(node.val)
            }
        }
    }

    pub fn push_back(&mut self, val: i32) {
        let mut node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            let mut old = self.tail.as_ref().prev.unwrap();
            self.tail.as_mut().prev = Some(node);
            node.as_mut().next = Some(self.tail);
            node.as_mut().prev = Some(old);
            old.as_mut().next = Some(node);

            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        unsafe {
            // if self.len == 0 {
            if self.tail.as_ref().prev == Some(self.head) {
                None
            } else {
                let old = self.tail.as_ref().prev.unwrap();
                let mut new = old.as_ref().prev.unwrap();
                self.tail.as_mut().prev = Some(new);
                new.as_mut().next = old.as_ref().next;

                self.len -= 1;

                let node = Box::from_raw(old.as_ptr());
                Some(node.val)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn predecessor_mut(&mut self, index: usize) -> Option<NonNull<Node>> {
        if index >= self.len {
            return None;
        }

        let mut iter = IterMut::new(Some(self.head), Some(self.tail), self.len + 1);
        if index < self.len - 1 - index {
            for _ in 0..index {
                iter.next();
            }
            iter.head
        } else {
            for _ in 0..self.len - 1 - index + 2 {
                iter.next_back();
            }
            iter.tail
        }
    }
}

impl Drop for MyLinkedList {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
        unsafe {
            let _ = Box::from_raw(self.head.as_ptr());
            let _ = Box::from_raw(self.tail.as_ptr());
        }
    }
}

#[derive(Debug)]
pub struct Node {
    val: i32,
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
}
