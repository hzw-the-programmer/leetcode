use crate::utils::binary_tree::*;
use std::cell::RefCell;
use std::rc::Rc;

// time  : O(n)
// space : O(n)
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    height(root.as_deref()) != -1
}

fn height(root: Option<&RefCell<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            let left = height(node.left.as_deref());
            let right = height(node.right.as_deref());
            if left == -1 || right == -1 {
                -1
            } else if (left - right).abs() > 1 {
                -1
            } else {
                1 + left.max(right)
            }
        }
    }
}
