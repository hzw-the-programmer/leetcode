use super::MyLinkedList;
use crate::utils::singly_linked_list::ListNode;

pub struct Iter<'a> {
    node: Option<&'a Box<ListNode>>,
}

impl<'a> Iter<'a> {
    fn new(node: Option<&'a Box<ListNode>>) -> Self {
        Self { node }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_ref();
            &node.val
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

impl MyLinkedList {
    pub fn iter(&self) -> Iter {
        Iter::new(self.dummy.next.as_ref())
    }
}
