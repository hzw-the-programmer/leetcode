use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => {
            {
                let mut node = node.borrow_mut();
                let left = node.left.take();
                let right = node.right.take();
                node.left = invert_tree(right);
                node.right = invert_tree(left);
            }
            Some(node)
        }
    }
}
