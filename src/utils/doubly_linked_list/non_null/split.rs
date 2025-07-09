use core::mem;
use core::ptr::NonNull;

use super::{LinkedList, Node};

impl<T> LinkedList<T> {
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
