use crate::utils::binary_tree::*;

pub fn recover_tree(root: &mut Tree) {
    let mut tree = root.clone();
    let mut pre: Tree = None;
    let (mut n1, mut n2) = (None, None);

    while let Some(node) = tree {
        if let Some(left) = node.borrow().left.clone() {
            let mut rightmost = left.clone();
            while let Some(right) = {
                let temp = rightmost.borrow().right.clone();
                temp
            } {
                if std::rc::Rc::ptr_eq(&right, &node) {
                    break;
                }
                rightmost = right;
            }

            if rightmost.borrow().right.is_none() {
                rightmost.borrow_mut().right = Some(node.clone());
                tree = Some(left);
                continue;
            } else {
                rightmost.borrow_mut().right = None;
                tree = node.borrow().right.clone();
            }
        } else {
            tree = node.borrow().right.clone();
        }

        if let Some(pre) = pre {
            if pre.borrow().val > node.borrow().val {
                n2 = Some(node.clone());
                if n1.is_none() {
                    n1 = Some(pre);
                }
            }
        }
        pre = Some(node);
    }

    match (n1, n2) {
        (Some(n1), Some(n2)) => {
            std::mem::swap(&mut n1.borrow_mut().val, &mut n2.borrow_mut().val);
        }
        _ => {}
    }
}
