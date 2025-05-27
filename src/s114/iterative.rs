use crate::utils::binary_tree::*;

pub fn flatten(root: &mut Tree) {
    let mut preordered = vec![];
    let mut stack = vec![];
    stack.push(root.clone());
    while let Some(tree) = stack.pop() {
        if let Some(node) = tree {
            stack.push(node.borrow_mut().right.take());
            stack.push(node.borrow_mut().left.take());
            preordered.push(node);
        }
    }

    for i in 0..preordered.len() - 1 {
        assert!(preordered[i].borrow_mut().left.is_none());
        assert!(preordered[i].borrow_mut().right.is_none());
        preordered[i].borrow_mut().right = Some(preordered[i + 1].clone());
    }
    assert!(preordered[preordered.len() - 1].borrow().left.is_none());
    assert!(preordered[preordered.len() - 1].borrow().right.is_none());
}
