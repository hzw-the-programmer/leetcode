use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut res = 0;
    let mut k = k;
    recursive(root.as_deref(), &mut k, &mut res);
    res
}

fn recursive(root: Option<&RefCell<TreeNode>>, k: &mut i32, res: &mut i32) {
    match root {
        None => {}
        Some(node) => {
            let node = node.borrow();
            recursive(node.left.as_deref(), k, res);
            *k -= 1;
            if *k == 0 {
                *res = node.val;
                return;
            } else if *k < 0 {
                return;
            }
            recursive(node.right.as_deref(), k, res);
        }
    }
}
