use crate::utils::binary_tree::*;

pub fn min_depth(root: Tree) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            match (node.left.clone(), node.right.clone()) {
                (None, None) => 1,
                (left, None) => 1 + min_depth(left),
                (None, right) => 1 + min_depth(right),
                (left, right) => {
                    let mut d = min_depth(left);
                    if d > 1 {
                        d = d.min(min_depth(right));
                    }
                    1 + d
                }
            }
        }
        None => 0,
    }
}
