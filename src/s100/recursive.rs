use crate::utils::binary_tree::*;

pub fn is_same_tree(p: Tree, q: Tree) -> bool {
    match (p, q) {
        (Some(p), Some(q)) => {
            let (p, q) = (p.borrow(), q.borrow());
            p.val == q.val
                && is_same_tree(p.left.clone(), q.left.clone())
                && is_same_tree(p.right.clone(), q.right.clone())
        }
        (None, None) => true,
        _ => false,
    }
}
