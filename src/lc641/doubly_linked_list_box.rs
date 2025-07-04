use core::ptr::NonNull;

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
    pre: Option<NonNull<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            pre: None,
        }
    }
}

pub struct MyCircularDeque {
    head: Option<Box<ListNode>>,
    tail: Option<NonNull<ListNode>>,
    capacity: usize,
    len: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        Self {
            head: None,
            tail: None,
            capacity: k as usize,
            len: 0,
        }
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let mut node = Box::new(ListNode::new(value));

        match self.head.take() {
            None => self.tail = Some(NonNull::from(&mut *node)),
            Some(mut head) => {
                head.pre = Some(NonNull::from(&mut *node));
                node.next = Some(head);
            }
        }

        self.head = Some(node);
        self.len += 1;

        true
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let mut node = Box::new(ListNode::new(value));
        let tail = self.tail;
        self.tail = Some(NonNull::from(&mut *node));

        unsafe {
            match tail {
                None => self.head = Some(node),
                Some(mut tail) => {
                    node.pre = Some(tail);
                    tail.as_mut().next = Some(node);
                }
            }
        }

        self.len += 1;
        true
    }

    pub fn delete_front(&mut self) -> bool {
        self.head
            .take()
            .map(|mut node| {
                self.head = node.next.take();

                match self.head.as_mut() {
                    None => self.tail = None,
                    Some(head) => head.pre = None,
                }

                self.len -= 1;
                node
            })
            .is_some()
    }

    pub fn delete_last(&mut self) -> bool {
        self.tail
            .map(|node| unsafe {
                self.tail = (*node.as_ptr()).pre;

                let node = match self.tail {
                    None => self.head.take().unwrap(),
                    Some(tail) => (*tail.as_ptr()).next.take().unwrap(),
                };

                self.len -= 1;
                node
            })
            .is_some()
    }

    pub fn get_front(&self) -> i32 {
        self.head.as_ref().map(|node| node.val).unwrap_or(-1)
    }

    pub fn get_rear(&self) -> i32 {
        self.tail
            .map(|node| unsafe { (*node.as_ptr()).val })
            .unwrap_or(-1)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}
