use crate::utils::binary_tree::*;

pub fn max_depth(root: Tree) -> i32 {
    match root {
        Some(node) => {
            1 + max_depth(node.borrow().left.clone()).max(max_depth(node.borrow().right.clone()))
        }
        None => 0,
    }
}
