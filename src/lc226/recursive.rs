use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => {
            {
                let mut b = node.borrow_mut();
                (b.left, b.right) = (invert_tree(b.right.take()), invert_tree(b.left.take()));
            }
            Some(node)
        }
    }
}
