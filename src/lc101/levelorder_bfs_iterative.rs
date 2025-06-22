use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        None => true,
        Some(node) => {
            let node = node.borrow();
            let mut stack = vec![(node.left.clone(), node.right.clone())];
            while let Some((left, right)) = stack.pop() {
                match (left, right) {
                    (None, None) => {}
                    (Some(_), None) | (None, Some(_)) => return false,
                    (Some(left), Some(right)) => {
                        let (left, right) = (left.borrow(), right.borrow());
                        if left.val != right.val {
                            return false;
                        } else {
                            stack.push((left.left.clone(), right.right.clone()));
                            stack.push((left.right.clone(), right.left.clone()));
                        }
                    }
                }
            }
            true
        }
    }
}
