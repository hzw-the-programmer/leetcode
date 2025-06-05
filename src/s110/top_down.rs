use crate::utils::binary_tree::*;
use std::cell::RefCell;
use std::rc::Rc;

// time  : O(n^2)
// space : O(n)
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    recursive(root.as_deref())
}

fn height(root: Option<&RefCell<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            let left = height(node.left.as_deref());
            let right = height(node.right.as_deref());
            1 + left.max(right)
        }
    }
}

fn recursive(root: Option<&RefCell<TreeNode>>) -> bool {
    match root {
        None => true,
        Some(node) => {
            let node = node.borrow();
            let left = height(node.left.as_deref());
            let right = height(node.right.as_deref());
            (left - right).abs() < 2
                && recursive(node.left.as_deref())
                && recursive(node.right.as_deref())
        }
    }
}
