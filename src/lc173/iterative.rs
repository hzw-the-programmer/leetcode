use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = Self { stack: Vec::new() };
        iter.load(root);
        iter
    }

    pub fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        self.load(node.borrow().right.clone());
        node.borrow().val
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn load(&mut self, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(node) = root {
            root = node.borrow().left.clone();
            self.stack.push(node);
        }
    }
}
