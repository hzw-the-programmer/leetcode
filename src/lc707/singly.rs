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
        self.iter().nth(index as usize).map_or(-1, |&val| val)
    }

    pub fn add_at_head(&mut self, val: i32) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            match self.head {
                None => self.tail = Some(node),
                Some(head) => (*node.as_ptr()).next = Some(head),
            }

            self.head = Some(node);
        }

        self.len += 1;
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

        unsafe {
            match self.tail {
                None => self.head = Some(node),
                Some(tail) => (*tail.as_ptr()).next = Some(node),
            }

            self.tail = Some(node);
        }

        self.len += 1;
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
            let mut iter = self.iter_mut();
            for _ in 0..index - 1 {
                iter.next();
            }
            let mut prev = iter.head.unwrap();
            let mut node = NonNull::from(Box::leak(Box::new(Node::new(val))));
            unsafe {
                node.as_mut().next = prev.as_ref().next;
                prev.as_mut().next = Some(node);
                match node.as_ref().next {
                    None => self.tail = Some(node),
                    _ => {}
                }
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
        } else {
            let mut iter = self.iter_mut();
            for _ in 0..index - 1 {
                iter.next();
            }
            let mut pre = iter.head.unwrap();
            unsafe {
                let cur = pre.as_ref().next.unwrap();
                let node = Box::from_raw(cur.as_ptr());
                pre.as_mut().next = node.next;
                match node.next {
                    None => self.tail = Some(pre),
                    _ => {}
                }
                self.len -= 1;
            }
        }
    }

    fn delete_at_head(&mut self) {
        self.head.map(|node| {
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            self.head = node.next;
            match self.head {
                None => self.tail = None,
                _ => {}
            }
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
}

impl Node {
    fn new(val: i32) -> Self {
        Self { val, next: None }
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
