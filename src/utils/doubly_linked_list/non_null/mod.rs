use core::cmp::Ordering;
use core::fmt;
use core::ptr::NonNull;

mod iter;
pub use iter::Iter;
mod into_iter;
mod iter_mut;

pub struct Node<T> {
    pub val: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));
        self.push_front_node(node);
    }

    pub fn push_back(&mut self, val: T) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));
        self.push_back_node(node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| {
            self.unlink(node);
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            node.val
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| {
            self.unlink(node);
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            node.val
        })
    }

    pub fn peek_front_node(&self) -> Option<NonNull<Node<T>>> {
        self.head
    }

    pub fn move_to_head(&mut self, node: NonNull<Node<T>>) {
        self.unlink(node);
        self.push_front_node(node);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).prev = None;
            (*node.as_ptr()).next = self.head;
            let node = Some(node);

            match self.head {
                None => self.tail = node,
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    pub fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).prev = self.tail;
            (*node.as_ptr()).next = None;
            let node = Some(node);

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    pub fn unlink(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            match ((*node.as_ptr()).prev, (*node.as_ptr()).next) {
                (None, None) => {
                    self.head = None;
                    self.tail = None;
                }
                (Some(prev), None) => {
                    (*prev.as_ptr()).next = None;
                    self.tail = Some(prev);
                }
                (None, Some(next)) => {
                    (*next.as_ptr()).prev = None;
                    self.head = Some(next);
                }
                (Some(prev), Some(next)) => {
                    (*prev.as_ptr()).next = Some(next);
                    (*next.as_ptr()).prev = Some(prev);
                }
            }
            self.len -= 1;
        }
    }

    pub fn link_after(&mut self, prev: NonNull<Node<T>>, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).prev = Some(prev);
            (*node.as_ptr()).next = (*prev.as_ptr()).next;
            let node = Some(node);
            match (*prev.as_ptr()).next {
                None => self.tail = node,
                Some(next) => (*next.as_ptr()).prev = node,
            }

            self.len += 1;
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

/////////////////////////////////////////////////////////////////////////
// PartialEq
/////////////////////////////////////////////////////////////////////////
impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

/////////////////////////////////////////////////////////////////////////
// PartialEq
/////////////////////////////////////////////////////////////////////////
impl<T: PartialEq> PartialEq<&[T]> for LinkedList<T> {
    fn eq(&self, other: &&[T]) -> bool {
        self.len() == other.len() && self.iter().eq(*other)
    }

    fn ne(&self, other: &&[T]) -> bool {
        self.len() != other.len() || self.iter().ne(*other)
    }
}

/////////////////////////////////////////////////////////////////////////
// PartialOrd
/////////////////////////////////////////////////////////////////////////
impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

/////////////////////////////////////////////////////////////////////////
// Extend
/////////////////////////////////////////////////////////////////////////
impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(|elem| self.push_back(elem));
    }
}

/////////////////////////////////////////////////////////////////////////
// Clone
/////////////////////////////////////////////////////////////////////////
impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut list = Self::new();
        list.extend(self.iter().cloned());
        list
    }
}

/////////////////////////////////////////////////////////////////////////
// FromIterator
/////////////////////////////////////////////////////////////////////////
impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter.into_iter());
        list
    }
}

/////////////////////////////////////////////////////////////////////////
// From
/////////////////////////////////////////////////////////////////////////
impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}

/////////////////////////////////////////////////////////////////////////
// Debug
/////////////////////////////////////////////////////////////////////////
impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
