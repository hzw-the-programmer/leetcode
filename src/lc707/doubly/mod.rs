use core::mem;
use core::ptr::NonNull;

mod into_iter;
mod iter;
mod iter_mut;

pub struct MyLinkedList {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
    len: usize,
}

impl MyLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.len as i32 {
            return -1;
        }

        let index = index as usize;
        let rindex = self.len - 1 - index;

        if index <= rindex {
            self.iter().nth(index).map_or(-1, |&n| n)
        } else {
            self.iter().nth_back(rindex).map_or(-1, |&n| n)
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

        if index > self.len {
            return;
        }

        let mut split = self.split_off(index);
        split.push_front(val);
        self.append(&mut split);
    }

    pub fn delete_at_index(&mut self, index: i32) {
        let index = index as usize;

        if index > self.len {
            return;
        }

        let mut split = self.split_off(index);
        split.pop_front();
        self.append(&mut split);
    }

    pub fn push_front(&mut self, val: i32) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            (*node.as_ptr()).next = self.head;
            (*node.as_ptr()).prev = None;
            let node = Some(node);

            match self.head {
                None => self.tail = node,
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;
            match self.head {
                None => self.tail = None,
                Some(head) => (*head.as_ptr()).prev = None,
            }
            self.len -= 1;
            node.val
        })
    }

    pub fn push_back(&mut self, val: i32) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = self.tail;
            let node = Some(node);

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;
            match self.tail {
                None => self.head = None,
                Some(tail) => (*tail.as_ptr()).next = None,
            }
            self.len -= 1;
            node.val
        })
    }

    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => mem::swap(self, other),
            Some(mut tail) => {
                if let Some(mut other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }

                    self.tail = other.tail.take();
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        assert!(at <= self.len);
        if at == 0 {
            return mem::replace(self, Self::new());
        } else if at == self.len {
            return Self::new();
        }

        let split_node = self.predecessor_mut(at);

        self.split_off_after_node(split_node, at)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn split_off_after_node(&mut self, split_node: Option<NonNull<Node>>, at: usize) -> Self {
        if let Some(mut split_node) = split_node {
            let second_part_head;
            let second_part_tail;
            unsafe {
                second_part_head = split_node.as_mut().next.take();
            }
            if let Some(mut head) = second_part_head {
                unsafe {
                    head.as_mut().prev = None;
                }
                second_part_tail = self.tail;
            } else {
                second_part_tail = None;
            }

            let second_part = Self {
                head: second_part_head,
                tail: second_part_tail,
                len: self.len - at,
            };

            self.tail = Some(split_node);
            self.len = at;

            second_part
        } else {
            mem::replace(self, Self::new())
        }
    }
}

impl Drop for MyLinkedList {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
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
