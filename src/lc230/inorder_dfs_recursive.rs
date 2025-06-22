use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut res = 0;
    let mut len = 0;
    recursive(root.as_deref(), k, &mut len, &mut res);
    res
}

fn recursive(root: Option<&RefCell<TreeNode>>, k: i32, len: &mut i32, res: &mut i32) {
    match root {
        None => {}
        Some(node) => {
            let node = node.borrow();
            recursive(node.left.as_deref(), k, len, res);
            *len += 1;
            if *len == k {
                *res = node.val;
                return;
            } else if *len > k {
                return;
            }
            recursive(node.right.as_deref(), k, len, res);
        }
    }
}
