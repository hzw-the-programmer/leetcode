use crate::utils::binary_tree::*;

pub fn recover_tree(root: &mut Tree) {
    let mut inordered = vec![];
    let mut stack = vec![];
    let mut tree = root.clone();
    while tree.is_some() || !stack.is_empty() {
        while let Some(node) = tree {
            tree = node.borrow().left.clone();
            stack.push(node);
        }
        if let Some(node) = stack.pop() {
            tree = node.borrow().right.clone();
            inordered.push(node);
        }
    }
    let (mut j, mut k) = (usize::MAX, 0);
    for i in 1..inordered.len() {
        if inordered[i - 1].borrow().val > inordered[i].borrow().val {
            if j == usize::MAX {
                j = i - 1;
                k = i;
            } else {
                k = i;
                break;
            }
        }
    }

    if j != usize::MAX {
        std::mem::swap(
            &mut inordered[j].borrow_mut().val,
            &mut inordered[k].borrow_mut().val,
        );
    }
}
