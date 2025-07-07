use core::marker::PhantomData;
use core::ptr::NonNull;
use std::collections::HashMap;

pub struct LRUCache {
    map: HashMap<i32, NonNull<Node<(i32, i32)>>>,
    list: LinkedList<(i32, i32)>,
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            list: LinkedList::new(),
            capacity: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.map.get(&key).map_or(-1, |&node| {
            self.list.move_to_head(node);
            unsafe { (*node.as_ptr()).val.1 }
        })
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.map
            .entry(key)
            .and_modify(|&mut node| {
                unsafe { (*node.as_ptr()).val.1 = value };
                self.list.move_to_head(node);
            })
            .or_insert_with(|| {
                self.list.push_front((key, value));
                self.list.peek_front_node().unwrap()
            });

        if self.list.len() > self.capacity {
            if let Some(val) = self.list.pop_back() {
                self.map.remove(&val.0);
            }
        }
    }

    pub fn iter(&self) -> Iter<(i32, i32)> {
        self.list.iter()
    }
}

struct Node<T> {
    val: T,
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

struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn move_to_head(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            match ((*node.as_ptr()).prev, (*node.as_ptr()).next) {
                (None, _) => return,
                (Some(prev), None) => {
                    (*prev.as_ptr()).next = None;
                    self.tail = Some(prev);
                }
                (Some(prev), Some(next)) => {
                    (*prev.as_ptr()).next = Some(next);
                    (*next.as_ptr()).prev = Some(prev);
                }
            }

            (*node.as_ptr()).prev = None;
            (*node.as_ptr()).next = self.head;
            match self.head {
                None => unreachable!(),
                Some(head) => (*head.as_ptr()).prev = Some(node),
            }
            self.head = Some(node);
        }
    }

    fn push_front(&mut self, val: T) {
        let node = NonNull::from(Box::leak(Box::new(Node::new(val))));

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

    fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;
            match self.tail {
                None => self.head = None,
                Some(prev) => (*prev.as_ptr()).next = None,
            }
            self.len -= 1;

            node.val
        })
    }

    fn peek_front_node(&self) -> Option<NonNull<Node<T>>> {
        self.head
    }

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> Iter<T> {
        Iter::new(self.head, self.tail, self.len)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

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
