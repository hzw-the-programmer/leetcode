use core::marker::PhantomData;
use core::mem;
use core::ptr::NonNull;

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
        self.iter().nth(index as usize).map_or(-1, |&n| n)
    }

    pub fn add_at_head(&mut self, val: i32) {
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

    pub fn add_at_tail(&mut self, val: i32) {
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

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        let index = index as usize;

        if index > self.len {
            return;
        }

        let mut split = self.split_off(index);
        split.add_at_head(val);
        self.append(&mut split);
    }

    pub fn delete_at_index(&mut self, index: i32) {
        let index = index as usize;

        if index > self.len {
            return;
        }

        let mut split = self.split_off(index);
        split.delete_at_head();
        self.append(&mut split);
    }

    pub fn delete_at_head(&mut self) {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;
            match self.head {
                None => self.tail = None,
                Some(head) => (*head.as_ptr()).prev = None,
            }
            self.len -= 1;
            node
        });
    }

    pub fn delete_at_tail(&mut self) {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;
            match self.tail {
                None => self.head = None,
                Some(tail) => (*tail.as_ptr()).next = None,
            }
            self.len -= 1;
            node
        });
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
        let len = self.len();
        assert!(at <= len);
        if at == 0 {
            return mem::replace(self, Self::new());
        } else if at == self.len {
            return Self::new();
        }

        let split_node = if at - 1 <= len - 1 - (at - 1) {
            let mut iter = self.iter_mut();
            for _ in 0..at - 1 {
                iter.next();
            }
            iter.head
        } else {
            let mut iter = self.iter_mut();
            for _ in 0..len - 1 - (at - 1) {
                iter.next_back();
            }
            iter.tail
        };

        self.split_off_after_node(split_node, at)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self.head)
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self.head, self.tail, self.len)
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
        while self.get(0) != -1 {
            self.delete_at_head()
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

pub struct Iter<'a> {
    head: Option<NonNull<Node>>,
    marker: PhantomData<&'a Node>,
}

impl<'a> Iter<'a> {
    fn new(head: Option<NonNull<Node>>) -> Self {
        Self {
            head,
            marker: PhantomData,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|node| unsafe {
            self.head = (*node.as_ptr()).next;
            &node.as_ref().val
        })
    }
}

impl<'a> IntoIterator for &'a MyLinkedList {
    type Item = &'a i32;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IterMut<'a> {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
    len: usize,
    marker: PhantomData<&'a mut Node>,
}

impl<'a> IterMut<'a> {
    fn new(head: Option<NonNull<Node>>, tail: Option<NonNull<Node>>, len: usize) -> Self {
        Self {
            head,
            tail,
            len,
            marker: PhantomData,
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|mut node| unsafe {
            let node = node.as_mut();
            self.head = node.next;
            self.len -= 1;
            &mut node.val
        })
    }
}

impl<'a> IntoIterator for &'a mut MyLinkedList {
    type Item = &'a mut i32;
    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a> DoubleEndedIterator for IterMut<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.map(|mut node| unsafe {
            let node = node.as_mut();
            self.tail = node.prev;
            self.len -= 1;
            &mut node.val
        })
    }
}
