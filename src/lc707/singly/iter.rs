use super::{MyLinkedList, Node};
use core::marker::PhantomData;
use core::ptr::NonNull;

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
            let node = node.as_ref();
            self.head = node.next;
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
        Iter::new(self.head)
    }
}
