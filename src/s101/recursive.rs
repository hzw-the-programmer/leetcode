use crate::utils::binary_tree::*;

pub fn is_symmetric(root: Tree) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        recursive(node.left.clone(), node.right.clone())
    } else {
        true
    }
}

fn recursive(l: Tree, r: Tree) -> bool {
    match (l, r) {
        (Some(l), Some(r)) => {
            let (mut l, mut r) = (l.borrow_mut(), r.borrow_mut());
            l.val == r.val
                && recursive(l.left.take(), r.right.take())
                && recursive(l.right.take(), r.left.take())
        }
        (None, None) => true,
        _ => false,
    }
}
