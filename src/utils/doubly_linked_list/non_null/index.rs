use core::ptr::NonNull;

use super::{LinkedList, Node};

impl<T> LinkedList<T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        let len = self.len();
        if index >= len {
            return None;
        }

        let rindex = len - 1 - index;

        if index <= rindex {
            self.iter().nth(index)
        } else {
            self.iter().nth_back(rindex)
        }
    }

    pub fn predecessor_mut(&mut self, index: usize) -> Option<NonNull<Node<T>>> {
        let len = self.len;
        if index == 0 || index > len {
            return None;
        }

        let mut iter = self.iter_mut();
        let pre_index = index - 1;
        // let pre_rindex = len - 1 - index + 1;
        let pre_rindex = len - index;

        if pre_index <= pre_rindex {
            for _ in 0..pre_index {
                iter.next();
            }
            iter.head
        } else {
            for _ in 0..pre_rindex {
                iter.next_back();
            }
            iter.tail
        }
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
}
