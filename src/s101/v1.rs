use crate::utils::binary_tree::*;

pub fn is_symmetric(root: Tree) -> bool {
    if let Some(node) = root {
        node.borrow().left == node.borrow().right
    } else {
        true
    }
}
