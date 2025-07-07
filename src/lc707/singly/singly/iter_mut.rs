use super::{MyLinkedList, Node};
use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct IterMut<'a> {
    head: Option<NonNull<Node>>,
    marker: PhantomData<&'a mut Node>,
}

impl<'a> IterMut<'a> {
    fn new(head: Option<NonNull<Node>>) -> Self {
        Self {
            head,
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
        IterMut::new(self.head)
    }
}

impl MyLinkedList {
    pub fn predecessor_mut(&mut self, index: usize) -> Option<NonNull<Node>> {
        if index > self.len {
            return None;
        }

        let mut iter = self.iter_mut();
        let pre_index = index - 1;
        for _ in 0..pre_index {
            iter.next();
        }
        iter.head
    }
}
