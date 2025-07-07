use core::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodePtr<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
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
    dummy: Rc<RefCell<Node<T>>>,
    len: usize,
}

impl<T: Default> LinkedList<T> {
    pub fn new() -> Self {
        let mut dummy = Rc::new(RefCell::new(Node::new(Default::default())));
        dummy.borrow_mut().prev = Some(Rc::downgrade(&dummy));
        Self { dummy, len: 0 }
    }

    pub fn push_front(&mut self, val: T) {
        let node = Rc::new(RefCell::new(Node::new(val)));

        node.borrow_mut().prev = Some(Rc::downgrade(&self.dummy));
        node.borrow_mut().next = self.dummy.next.take();

        if node.borrow().next.is_none() {
            self.dummy.prev = Some(Rc::downgrade(&node));
        }

        self.dummy.next = Some(node);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        None
    }

    pub fn move_to_head(&mut self, node: Rc<RefCell<Node<T>>>) {}

    pub fn peek_front_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        None
    }

    pub fn move_to_head(&mut self, node: Rc<RefCell<Node<T>>>) {}

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self.dummy.next.as_ref(), self.dummy.prev.as_ref(), self.len)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Rc<RefCell<Node<T>>>>,
    prev: Option<&'a Weak<RefCell<Node<T>>>>,
    len: usize,
}

impl<'a, T> Iter<'a, T> {
    fn new(
        head: Option<&'a Rc<RefCell<Node<T>>>>,
        tail: Option<&'a Weak<RefCell<Node<T>>>>,
        len: usize,
    ) -> Self {
        Self { head, tail, len }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.next.map(|node| unsafe {
                self.next = node.next.as_ref();
                self.len -= 1;
                &node.val
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.prev.map(|node| unsafe {
                let node = node.upgrade().unwrap();
                self.prev = node.borrow().prev.as_ref();
                self.len -= 1;
                &node.borrow().val
            })
        }
    }
}
