use crate::utils::doubly_linked_list::{IntoIter, Iter, IterMut, LinkedList};

pub struct MyLinkedList {
    list: LinkedList<i32>,
}

impl MyLinkedList {
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        self.list.get(index as usize).map_or(-1, |&n| n)
    }

    pub fn add_at_head(&mut self, val: i32) {
        self.list.push_front(val);
    }

    pub fn add_at_tail(&mut self, val: i32) {
        self.list.push_back(val);
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        self.list.add_at_index(index as usize, val);
    }

    pub fn delete_at_index(&mut self, index: i32) {
        self.list.delete_at_index(index as usize);
    }
}

impl MyLinkedList {
    pub fn iter(&self) -> Iter<i32> {
        self.list.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<i32> {
        self.list.iter_mut()
    }

    pub fn into_iter(self) -> IntoIter<i32> {
        self.list.into_iter()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl<'a> IntoIterator for &'a MyLinkedList {
    type Item = &'a i32;
    type IntoIter = Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut MyLinkedList {
    type Item = &'a mut i32;
    type IntoIter = IterMut<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl IntoIterator for MyLinkedList {
    type Item = i32;
    type IntoIter = IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}
