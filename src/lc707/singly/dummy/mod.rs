use crate::utils::singly_linked_list::ListNode;

mod into_iter;
mod iter;
mod iter_mut;

pub struct MyLinkedList {
    dummy: Box<ListNode>,
    len: usize,
}

impl MyLinkedList {
    pub fn new() -> Self {
        Self {
            dummy: Box::new(ListNode::new(0)),
            len: 0,
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        self.iter().nth(index as usize).map_or(-1, |&val| val)
    }

    pub fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }

    pub fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.len as i32, val);
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if let Some(predecessor) = self.predecessor_mut(index as usize) {
            let mut node = Box::new(ListNode::new(val));
            node.next = predecessor.next.take();
            predecessor.next = Some(node);
            self.len += 1;
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if let Some(predecessor) = self.predecessor_mut(index as usize) {
            if let Some(node) = predecessor.next.take() {
                predecessor.next = node.next;
                self.len -= 1;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        self.dummy.next.take().map(|mut node| {
            self.dummy.next = node.next.take();
            node.val
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
