use crate::utils::binary_tree::*;

pub fn is_same_tree(p: Tree, q: Tree) -> bool {
    let mut stack = vec![(p, q)];
    while let Some((p, q)) = stack.pop() {
        match (p, q) {
            (Some(p), Some(q)) => {
                let (mut p, mut q) = (p.borrow_mut(), q.borrow_mut());
                if p.val != q.val {
                    return false;
                }
                stack.push((p.right.take(), q.right.take()));
                stack.push((p.left.take(), q.left.take()));
            }
            (None, None) => {}
            _ => return false,
        }
    }
    true
}
