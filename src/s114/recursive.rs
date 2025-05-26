use crate::binary_tree::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn flatten(root: &mut Tree) {
    let mut preordered = vec![];
    preorder_traversal(root, &mut preordered);
    for i in 0..preordered.len() {
        preordered[i].borrow_mut().left = None;
        if i == preordered.len() - 1 {
            preordered[i].borrow_mut().right = None;
        } else {
            preordered[i].borrow_mut().right = Some(preordered[i + 1].clone());
        }
    }
}

fn preorder_traversal(root: &Tree, res: &mut Vec<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        res.push(node.clone());
        preorder_traversal(&node.borrow().left, res);
        preorder_traversal(&node.borrow().right, res);
    }
}
