use crate::utils::binary_tree::*;

pub fn recover_tree(root: &mut Tree) {
    let mut tree = root.clone();
    let mut stack = vec![];
    let mut pre: Tree = None;
    let (mut n1, mut n2) = (None, None);
    while tree.is_some() || !stack.is_empty() {
        while let Some(node) = tree {
            tree = node.borrow().left.clone();
            stack.push(node);
        }
        if let Some(node) = stack.pop() {
            tree = node.borrow().right.clone();
            if let Some(pre) = pre {
                if pre.borrow().val > node.borrow().val {
                    if n1.is_none() {
                        n1 = Some(pre);
                        n2 = Some(node.clone());
                    } else {
                        n2 = Some(node.clone());
                        break;
                    }
                }
            }
            pre = Some(node);
        }
    }

    match (n1, n2) {
        (Some(n1), Some(n2)) => {
            std::mem::swap(&mut n1.borrow_mut().val, &mut n2.borrow_mut().val);
        }
        _ => {}
    }
}
