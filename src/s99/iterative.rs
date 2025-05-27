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
    let mut j = usize::MAX;
    let mut k = usize::MAX;
    for i in 1..inordered.len() {
        if inordered[i].borrow().val < inordered[i - 1].borrow().val {
            if j == usize::MAX {
                j = i - 1;
                k = i;
            } else {
                k = i;
                break;
            }
        }
    }

    let temp = inordered[j].borrow().val;
    inordered[j].borrow_mut().val = inordered[k].borrow().val;
    inordered[k].borrow_mut().val = temp;
}
