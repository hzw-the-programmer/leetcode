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

        unsafe {
            let next = (*self.dummy.as_ptr()).next.unwrap();
            let prev = (*next.as_ptr()).prev.unwrap();

            (*node.as_ptr()).next = Some(next);
            (*node.as_ptr()).prev = Some(prev);

            (*prev.as_ptr()).next = Some(node);
            (*next.as_ptr()).prev = Some(node);

            self.len += 1;
        }
    }

    pub fn push_back(&mut self, val: T) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            let prev = (*self.dummy.as_ptr()).prev.unwrap();
            let next = (*prev.as_ptr()).next.unwrap();

            (*node.as_ptr()).next = Some(next);
            (*node.as_ptr()).prev = Some(prev);

            (*prev.as_ptr()).next = Some(node);
            (*next.as_ptr()).prev = Some(node);

            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            (*self.dummy.as_ptr()).next.map(|node| {
                let node = Box::from_raw(node.as_ptr());

                let next = node.next.unwrap();
                let prev = node.prev.unwrap();

                (*prev.as_ptr()).next = Some(next);
                (*next.as_ptr()).prev = Some(prev);

                self.len -= 1;

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
                let node = Box::from_raw(node.as_ptr());

                let next = node.next.unwrap();
                let prev = node.prev.unwrap();

                (*prev.as_ptr()).next = Some(next);
                (*next.as_ptr()).prev = Some(prev);

                self.len -= 1;

                node.val
            })
        }
    }

    pub fn peek_front_node(&self) -> Option<NonNull<Node<T>>> {
        unsafe { (*self.dummy.as_ptr()).next }
    }

    pub fn move_to_head(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            // remove
            let prev = (*node.as_ptr()).prev.unwrap();
            let next = (*node.as_ptr()).next.unwrap();

            (*prev.as_ptr()).next = Some(next);
            (*next.as_ptr()).prev = Some(prev);

            // add head
            let next = (*self.dummy.as_ptr()).next.unwrap();
            let prev = (*next.as_ptr()).prev.unwrap();

            (*node.as_ptr()).next = Some(next);
            (*node.as_ptr()).prev = Some(prev);

            (*prev.as_ptr()).next = Some(node);
            (*next.as_ptr()).prev = Some(node);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
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
