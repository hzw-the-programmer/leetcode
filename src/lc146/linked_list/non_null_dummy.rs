use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ptr::NonNull;

pub type NodePtr<T> = NonNull<Node<T>>;

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

#[derive(Debug)]
pub struct LinkedList<T> {
    dummy: NonNull<Node<T>>,
    len: usize,
}

impl<T: Default> LinkedList<T> {
    pub fn new() -> Self {
        let dummy = NonNull::from(Box::leak(Box::new(Node::new(Default::default()))));

        unsafe {
            (*dummy.as_ptr()).prev = Some(dummy);
            (*dummy.as_ptr()).next = Some(dummy);
        }

        Self { dummy, len: 0 }
    }
}

impl<T> LinkedList<T> {
    pub fn push_front(&mut self, val: T) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        self.link_after(self.dummy, node);
    }

    pub fn push_back(&mut self, val: T) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            self.link_after((*self.dummy.as_ptr()).prev.unwrap(), node);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            (*self.dummy.as_ptr()).next.map(|node| {
                self.unlink(node);

                let node = Box::from_raw(node.as_ptr());
                node.val
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            (*self.dummy.as_ptr()).prev.map(|node| {
                self.unlink(node);

                let node = Box::from_raw(node.as_ptr());
                node.val
            })
        }
    }

    pub fn peek_front_node(&self) -> Option<NonNull<Node<T>>> {
        unsafe { (*self.dummy.as_ptr()).next }
    }

    pub fn move_to_head(&mut self, node: NonNull<Node<T>>) {
        self.unlink(node);
        self.link_after(self.dummy, node);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn unlink(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            let prev = (*node.as_ptr()).prev.unwrap();
            let next = (*node.as_ptr()).next.unwrap();

            (*prev.as_ptr()).next = Some(next);
            (*next.as_ptr()).prev = Some(prev);

            self.len -= 1;
        }
    }

    fn link_after(&mut self, prev: NonNull<Node<T>>, node: NonNull<Node<T>>) {
        unsafe {
            let next = (*prev.as_ptr()).next.unwrap();

            (*node.as_ptr()).next = Some(next);
            (*node.as_ptr()).prev = Some(prev);

            (*prev.as_ptr()).next = Some(node);
            (*next.as_ptr()).prev = Some(node);

            self.len += 1;
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
        let _ = unsafe { Box::from_raw(self.dummy.as_ptr()) };
    }
}

/////////////////////////////////////////////////////////////////////////
// Iter
/////////////////////////////////////////////////////////////////////////
pub struct Iter<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iter<'a, T> {
    fn new(head: Option<NonNull<Node<T>>>, tail: Option<NonNull<Node<T>>>, len: usize) -> Self {
        Self {
            head,
            tail,
            len,
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                self.head = (*node.as_ptr()).next;
                self.len -= 1;
                &(*node.as_ptr()).val
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                self.tail = (*node.as_ptr()).prev;
                self.len -= 1;
                &(*node.as_ptr()).val
            })
        }
    }
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        unsafe {
            Iter::new(
                (*self.dummy.as_ptr()).next,
                (*self.dummy.as_ptr()).prev,
                self.len,
            )
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/////////////////////////////////////////////////////////////////////////
// IterMut
/////////////////////////////////////////////////////////////////////////
pub struct IterMut<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> IterMut<'a, T> {
    fn new(head: Option<NonNull<Node<T>>>, tail: Option<NonNull<Node<T>>>, len: usize) -> Self {
        Self {
            head,
            tail,
            len,
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                self.head = (*node.as_ptr()).next;
                self.len -= 1;
                &mut (*node.as_ptr()).val
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                self.tail = (*node.as_ptr()).prev;
                self.len -= 1;
                &mut (*node.as_ptr()).val
            })
        }
    }
}

impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        unsafe {
            IterMut::new(
                (*self.dummy.as_ptr()).next,
                (*self.dummy.as_ptr()).prev,
                self.len,
            )
        }
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/////////////////////////////////////////////////////////////////////////
// IntoIter
/////////////////////////////////////////////////////////////////////////
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> IntoIter<T> {
    fn new(list: LinkedList<T>) -> Self {
        Self { list }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
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
impl<T: Clone + Default> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut list = Self::new();
        list.extend(self.iter().cloned());
        list
    }
}

/////////////////////////////////////////////////////////////////////////
// FromIterator
/////////////////////////////////////////////////////////////////////////
impl<T: Default> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter.into_iter());
        list
    }
}

/////////////////////////////////////////////////////////////////////////
// From
/////////////////////////////////////////////////////////////////////////
impl<T: Default, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
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
