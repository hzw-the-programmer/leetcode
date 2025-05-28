use crate::utils::binary_tree::*;

pub fn max_depth(root: Tree) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        1 + max_depth(node.left.clone()).max(max_depth(node.right.clone()))
    } else {
        0
    }
}
