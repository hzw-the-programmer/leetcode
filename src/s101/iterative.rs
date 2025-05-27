use crate::utils::binary_tree::*;

pub fn is_symmetric(root: Tree) -> bool {
    if let Some(node) = root {
        let mut stack = vec![(node.borrow().left.clone(), node.borrow().right.clone())];
        while let Some(pair) = stack.pop() {
            match pair {
                (Some(l), Some(r)) => {
                    if l.borrow().val != r.borrow().val {
                        return false;
                    }
                    stack.push((l.borrow().left.clone(), r.borrow().right.clone()));
                    stack.push((l.borrow().right.clone(), r.borrow().left.clone()));
                }
                (None, None) => {}
                _ => return false,
            }
        }
    }
    true
}
