use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub struct BSTIterator {
    inorder: Vec<i32>,
    idx: usize,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = Self {
            inorder: vec![],
            idx: 0,
        };
        Self::recursive(root.as_deref(), &mut iter.inorder);
        iter
    }

    pub fn next(&mut self) -> i32 {
        let idx = self.idx;
        self.idx += 1;
        self.inorder[idx]
    }

    pub fn has_next(&self) -> bool {
        self.idx < self.inorder.len()
    }

    fn recursive(root: Option<&RefCell<TreeNode>>, res: &mut Vec<i32>) {
        match root {
            None => {}
            Some(node) => {
                Self::recursive(node.borrow().left.as_deref(), res);
                res.push(node.borrow().val);
                Self::recursive(node.borrow().right.as_deref(), res);
            }
        }
    }
}
