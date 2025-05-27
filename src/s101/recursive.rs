use crate::utils::binary_tree::*;

pub fn is_symmetric(root: Tree) -> bool {
    if let Some(node) = &root {
        let node = node.borrow();
        recursive(&node.left, &node.right)
    } else {
        true
    }
}

fn recursive(l: &Tree, r: &Tree) -> bool {
    match (l, r) {
        (Some(l), Some(r)) => {
            let (l, r) = (l.borrow(), r.borrow());
            l.val == r.val && recursive(&l.left, &r.right) && recursive(&l.right, &r.left)
        }
        (None, None) => true,
        _ => false,
    }
}
