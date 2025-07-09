use core::mem;
use core::ptr::NonNull;

use super::{LinkedList, Node};

impl<T> LinkedList<T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        self.get_node(index)
            .map(|node| unsafe { &(*node.as_ptr()).val })
    }

    pub fn add_at_index(&mut self, index: usize, val: T) {
        if index > self.len() {
            return;
        }

        let mut split = self.split_off(index);
        split.push_front(val);
        self.append(&mut split);
    }

    pub fn delete_at_index(&mut self, index: usize) {
        if index >= self.len() {
            return;
        }

        let mut split = self.split_off(index);
        split.pop_front();
        self.append(&mut split);
    }

    pub fn get_node(&self, index: usize) -> Option<NonNull<Node<T>>> {
        let len = self.len();
        if index >= len {
            return None;
        }

        let rindex = len - 1 - index;

        let mut iter = self.iter();

        if index <= rindex {
            for _ in 0..index {
                iter.next();
            }
            iter.head
        } else {
            for _ in 0..rindex {
                iter.next_back();
            }
            iter.tail
        }
    }

    pub fn get_prev_node(&self, index: usize) -> Option<NonNull<Node<T>>> {
        if index == 0 {
            None
        } else {
            self.get_node(index - 1)
        }
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        assert!(at <= self.len);
        if at == 0 {
            return mem::replace(self, Self::new());
        } else if at == self.len {
            return Self::new();
        }

        let split_node = self.get_prev_node(at);

        self.split_off_after_node(split_node, at)
    }

    fn split_off_after_node(&mut self, split_node: Option<NonNull<Node<T>>>, at: usize) -> Self {
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
