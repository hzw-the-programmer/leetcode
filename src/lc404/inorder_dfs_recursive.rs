use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    match root {
        None => {}
        Some(root) => {
            let root = root.borrow();
            if let Some(node) = root.left.clone() {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    res += node.val;
                } else {
                    res += sum_of_left_leaves(root.left.clone());
                }
            }
            res += sum_of_left_leaves(root.right.clone());
        }
    }
    res
}
