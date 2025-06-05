use crate::utils::binary_tree::*;
use std::cell::RefCell;
use std::rc::Rc;

// time  : O(n)
// space : O(n)
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    check_balanced(root.as_deref()).is_ok()
}

fn check_balanced(root: Option<&RefCell<TreeNode>>) -> Result<i32, ()> {
    match root {
        None => Ok(0),
        Some(node) => {
            let node = node.borrow();
            let left = check_balanced(node.left.as_deref())?;
            let right = check_balanced(node.right.as_deref())?;
            if (left - right).abs() > 1 {
                Err(())
            } else {
                Ok(1 + left.max(right))
            }
        }
    }
}
