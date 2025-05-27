use crate::utils::binary_tree::*;

pub fn is_symmetric(root: Tree) -> bool {
    if let Some(node) = root {
        let mut node = node.borrow_mut();
        let mut stack = vec![(node.left.take(), node.right.clone())];
        while let Some(pair) = stack.pop() {
            match pair {
                (Some(l), Some(r)) => {
                    let (mut l, mut r) = (l.borrow_mut(), r.borrow_mut());
                    if l.val != r.val {
                        return false;
                    }
                    stack.push((l.left.take(), r.right.take()));
                    stack.push((l.right.take(), r.left.take()));
                }
                (None, None) => {}
                _ => return false,
            }
        }
    }
    true
}
