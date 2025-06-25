use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use core::ptr;
use std::rc::Rc;

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let (mut depth, mut max_depth, mut leaf_count) = (-1, -1, 0);
    let mut current = root;
    let mut stack = vec![];
    let mut pre = ptr::null();
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current.take() {
            current = node.borrow().left.clone();
            stack.push(node);
            depth += 1;
        }

        if let Some(node) = stack.pop() {
            if max_depth == -1 {
                max_depth = depth;
            }

            if node.borrow().right.is_none()
                || ptr::eq(node.borrow().right.as_deref().unwrap(), pre)
            {
                if node.borrow().right.is_none() {
                    if depth == max_depth {
                        leaf_count += 1;
                    } else {
                        break;
                    }
                }
                depth -= 1;
                pre = Rc::as_ptr(&node);
            } else {
                current = node.borrow().right.clone();
                stack.push(node);
            }
        }
    }

    match max_depth {
        -1 => 0,
        0 => 1,
        n => (1 << n) - 1 + leaf_count,
    }
}
