use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    recursive(root.as_deref(), target_sum, 0)
}

fn recursive(root: Option<&RefCell<TreeNode>>, target_sum: i32, sum: i32) -> bool {
    match root {
        None => false,
        Some(node) => {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() && node.val + sum == target_sum {
                true
            } else {
                recursive(node.left.as_deref(), target_sum, node.val + sum)
                    || recursive(node.right.as_deref(), target_sum, node.val + sum)
            }
        }
    }
}
