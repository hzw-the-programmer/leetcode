use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub struct BSTIterator {
    current: Option<Rc<RefCell<TreeNode>>>,
    links: usize,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            current: root,
            links: 0,
        }
    }

    pub fn next(&mut self) -> i32 {
        let mut res = 0;
        let mut current = self.current.take();
        while let Some(node) = current {
            if let Some(left) = node.borrow().left.clone() {
                let mut rightmost = left.clone();
                while rightmost.borrow().right.is_some() {
                    let right = rightmost.borrow().right.clone().unwrap();
                    if Rc::ptr_eq(&right, &node) {
                        break;
                    }
                    rightmost = right;
                }
                if rightmost.borrow().right.is_none() {
                    rightmost.borrow_mut().right = Some(node.clone());
                    current = Some(left);
                    self.links += 1;
                } else {
                    rightmost.borrow_mut().right = None;
                    current = node.borrow().right.clone();
                    res = node.borrow().val;
                    self.links -= 1;
                    break;
                }
            } else {
                current = node.borrow().right.clone();
                res = node.borrow().val;
                break;
            }
        }

        self.current = current;
        res
    }

    pub fn has_next(&self) -> bool {
        self.current.is_some()
    }
}

impl Drop for BSTIterator {
    fn drop(&mut self) {
        while self.links != 0 {
            self.next();
        }
    }
}
