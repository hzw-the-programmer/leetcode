use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    recursive(root.as_deref(), 0, &mut res);
    res
}

fn recursive(root: Option<&RefCell<TreeNode>>, mut n: i32, res: &mut i32) {
    if let Some(node) = root {
        n <<= 1;
        n |= node.borrow().val;
        match (
            node.borrow().left.as_deref(),
            node.borrow().right.as_deref(),
        ) {
            (None, None) => *res += n,
            (left, right) => {
                recursive(left, n, res);
                recursive(right, n, res);
            }
        }
    }
}
