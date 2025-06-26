use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    match root {
        None => {}
        Some(root) => {
            let mut stack = vec![root];
            while let Some(node) = stack.pop() {
                stack.extend(node.borrow().right.clone());
                if let Some(left) = node.borrow().left.clone() {
                    if left.borrow().left.is_none() && left.borrow().right.is_none() {
                        res += left.borrow().val;
                    } else {
                        stack.push(left);
                    }
                }
            }
        }
    }

    res
}
