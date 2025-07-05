use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct MyLinkedList {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
    len: usize,
}

impl MyLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        self.iter().nth(index as usize).map_or(-1, |node| node.val)
    }

    pub fn add_at_head(&mut self, val: i32) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            (*node.as_ptr()).next = self.head;
            (*node.as_ptr()).prev = None;
            let node = Some(node);

            match self.head {
                None => self.tail = node,
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = self.tail;
            let node = Some(node);

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        let index = index as usize;

        if index > self.len {
            return;
        }

        if index == 0 {
            self.add_at_head(val);
        } else if index == self.len {
            self.add_at_tail(val);
        } else {
            if let Some(prev) = self.iter_mut().nth(index - 1) {
                let node = NonNull::from(Box::leak(Box::new(Node::new(val))));
                unsafe { (*node.as_ptr()).next = prev.next };
                prev.next = Some(node);
                self.len += 1;
            }
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        let index = index as usize;

        if index >= self.len {
            return;
        }

        if index == 0 {
            self.delete_at_head();
        } else if index == self.len - 1 {
            self.delete_at_tail();
        } else {
            if let Some(prev) = self.iter_mut().nth(index - 1) {
                unsafe {
                    let node = Box::from_raw(prev.next.unwrap().as_ptr());
                    (*node.next.unwrap().as_ptr()).prev = node.prev;
                    prev.next = node.next;
                    self.len -= 1;
                }
            }
        }
    }

    fn delete_at_head(&mut self) {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;
            match self.head {
                None => self.tail = None,
                Some(head) => (*head.as_ptr()).prev = None,
            }
            self.len -= 1;
            node
        });
    }

    fn delete_at_tail(&mut self) {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;
            match self.tail {
                None => self.head = None,
                Some(tail) => (*tail.as_ptr()).next = None,
            }
            self.len -= 1;
            node
        });
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self.head)
    }

    fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self.head)
    }
}

impl Drop for MyLinkedList {
    fn drop(&mut self) {
        while self.get(0) != -1 {
            self.delete_at_head()
        }
    }
}

#[derive(Debug)]
pub struct Node {
    val: i32,
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
}

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
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|node| unsafe {
            self.head = (*node.as_ptr()).next;
            node.as_ref()
        })
    }
}

impl<'a> IntoIterator for &'a MyLinkedList {
    type Item = &'a Node;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct IterMut<'a> {
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
    type Item = &'a mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.map(|mut node| unsafe {
            self.head = (*node.as_ptr()).next;
            node.as_mut()
        })
    }
}
