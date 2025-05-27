use crate::utils::binary_tree::*;

pub fn is_same_tree(p: Tree, q: Tree) -> bool {
    recursive(p, q)
}

fn recursive(p: Tree, q: Tree) -> bool {
    match (p, q) {
        (Some(p_node), Some(q_node)) => {
            if p_node.borrow().val == q_node.borrow().val {
                recursive(p_node.borrow().left.clone(), q_node.borrow().left.clone())
                    && recursive(p_node.borrow().right.clone(), q_node.borrow().right.clone())
            } else {
                false
            }
        }
        (None, None) => true,
        _ => false,
    }
}
