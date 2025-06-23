use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
    match root {
        None => {}
        Some(node) => {
            let mut stack = vec![node.clone()];
            let mut current = Some(node);
            while !stack.is_empty() || current.is_some() {
                while let Some(node) = current {
                    stack.push(node.clone());
                    current = node.borrow().left.clone();
                }

                if let Some(node) = stack.pop() {
                    let node = node.borrow();
                    k -= 1;
                    if k == 0 {
                        return node.val;
                    }
                    current = node.right.clone();
                }
            }
        }
    }
    0
}
