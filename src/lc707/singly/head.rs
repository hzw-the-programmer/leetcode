use crate::utils::singly_linked_list::ListNode;

pub struct MyLinkedList {
    head: Option<Box<ListNode>>,
    len: usize,
}

impl MyLinkedList {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn get(&self, index: i32) -> i32 {
        self.iter().nth(index as usize).map_or(-1, |&val| val)
    }

    pub fn add_at_head(&mut self, val: i32) {
        let mut node = Box::new(ListNode::new(val));
        node.next = self.head.take();
        self.head = Some(node);
        self.len += 1;
    }

    pub fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.len as i32, val);
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if let Some(predecessor) = self.predecessor_mut(index) {
            let mut node = Box::new(ListNode::new(val));
            node.next = predecessor.next.take();
            predecessor.next = Some(node);
            self.len += 1;
        } else if index == 0 {
            self.add_at_head(val);
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if let Some(predecessor) = self.predecessor_mut(index) {
            if let Some(node) = predecessor.next.take() {
                predecessor.next = node.next;
                self.len -= 1;
            }
        } else if index == 0 {
            self.delete_at_head();
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self.head.as_ref())
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self.head.as_mut())
    }

    fn predecessor_mut(&mut self, index: i32) -> Option<&mut Box<ListNode>> {
        if index <= 0 || index > self.len as i32 {
            return None;
        }

        let mut iter = self.iter_mut();
        for _ in 0..index - 1 {
            iter.next();
        }
        iter.node
    }

    fn delete_at_head(&mut self) {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.len -= 1;
        });
    }
}

pub struct Iter<'a> {
    node: Option<&'a Box<ListNode>>,
}

impl<'a> Iter<'a> {
    fn new(node: Option<&'a Box<ListNode>>) -> Self {
        Self { node }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_ref();
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
    node: Option<&'a mut Box<ListNode>>,
}

impl<'a> IterMut<'a> {
    fn new(node: Option<&'a mut Box<ListNode>>) -> Self {
        Self { node }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_mut();
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
