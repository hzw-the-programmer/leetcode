use super::{MyLinkedList, Node};
use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct IterMut<'a> {
    pub(super) head: Option<NonNull<Node>>,
    pub(super) tail: Option<NonNull<Node>>,
    len: usize,
    marker: PhantomData<&'a mut Node>,
}

impl<'a> IterMut<'a> {
    pub(super) fn new(
        head: Option<NonNull<Node>>,
        tail: Option<NonNull<Node>>,
        len: usize,
    ) -> Self {
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
        if self.len == 0 {
            None
        } else {
            self.head.map(|mut node| unsafe {
                let node = node.as_mut();
                self.head = node.next;
                self.len -= 1;
                &mut node.val
            })
        }
    }
}

impl<'a> DoubleEndedIterator for IterMut<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|mut node| unsafe {
                let node = node.as_mut();
                self.tail = node.prev;
                self.len -= 1;
                &mut node.val
            })
        }
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
        unsafe { IterMut::new(self.head.as_ref().next, self.tail.as_ref().prev, self.len) }
    }
}
