use core::ptr::NonNull;

use super::{LinkedList, Node};

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            prev: None,
            next: None,
        }
    }
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
        self.push_front_node(node);
    }

    pub fn push_back(&mut self, val: T) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));
        self.push_back_node(node);
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

    pub fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        self.link_after(self.dummy, node);
    }

    pub fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            self.link_after((*self.dummy.as_ptr()).prev.unwrap(), node);
        }
    }

    pub fn unlink(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            let prev = (*node.as_ptr()).prev.unwrap();
            let next = (*node.as_ptr()).next.unwrap();

            (*prev.as_ptr()).next = Some(next);
            (*next.as_ptr()).prev = Some(prev);

            self.len -= 1;
        }
    }

    pub fn link_after(&mut self, prev: NonNull<Node<T>>, node: NonNull<Node<T>>) {
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
