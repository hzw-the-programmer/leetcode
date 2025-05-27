use crate::utils::binary_tree::*;

pub fn is_valid_bst(root: Tree) -> bool {
    recursive(root, i32::MIN, i32::MAX)
}

fn recursive(root: Tree, lower: i32, upper: i32) -> bool {
    if let Some(node) = &root {
        let node = node.borrow();
        if node.val <= lower || node.val >= upper {
            false
        } else {
            recursive(node.left.clone(), lower, node.val)
                && recursive(node.right.clone(), node.val, upper)
        }
    } else {
        true
    }
}
