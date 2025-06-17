use crate::utils::binary_tree::*;

pub fn is_symmetric(root: Tree) -> bool {
    if let Some(node) = &root {
        let node = node.borrow();
        let mut stack = vec![(node.left.clone(), node.right.clone())];
        while let Some(pair) = stack.pop() {
            match pair {
                (Some(l), Some(r)) => {
                    let (l, r) = (l.borrow(), r.borrow());
                    if l.val != r.val {
                        return false;
                    }
                    stack.push((l.right.clone(), r.left.clone()));
                    stack.push((l.left.clone(), r.right.clone()));
                }
                (None, None) => {}
                _ => return false,
            }
        }
    }
    true
}
