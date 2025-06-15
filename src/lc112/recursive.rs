use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// time : O(n)
// space: O(h)
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    recursive(root.as_deref(), target_sum)
}

fn recursive(root: Option<&RefCell<TreeNode>>, target_sum: i32) -> bool {
    match root {
        None => false,
        Some(node) => {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                node.val == target_sum
            } else {
                recursive(node.left.as_deref(), target_sum - node.val)
                    || recursive(node.right.as_deref(), target_sum - node.val)
            }
        }
    }
}
