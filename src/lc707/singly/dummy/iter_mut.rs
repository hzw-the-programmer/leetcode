use super::MyLinkedList;
use crate::utils::singly_linked_list::ListNode;

pub struct IterMut<'a> {
    node: Option<&'a mut Box<ListNode>>,
}

impl<'a> IterMut<'a> {
    fn new(node: Option<&'a mut Box<ListNode>>) -> Self {
        Self { node }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_mut();
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

impl MyLinkedList {
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self.dummy.next.as_mut())
    }
}

impl MyLinkedList {
    pub fn predecessor_mut(&mut self, index: usize) -> Option<&mut Box<ListNode>> {
        if index > self.len {
            return None;
        }

        let mut iter = IterMut::new(Some(&mut self.dummy));
        let pre_index = index;

        for _ in 0..pre_index {
            iter.next();
        }
        iter.node
    }
}
