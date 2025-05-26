use crate::utils::binary_tree::*;

pub fn flatten(root: &mut Tree) {
    let mut root = root.clone();
    while let Some(node) = root {
        if node.borrow().left.is_some() {
            let mut cur = node.borrow().left.clone().unwrap();
            while cur.borrow().right.is_some() {
                let n = cur.borrow().right.clone().unwrap();
                cur = n;
            }
            cur.borrow_mut().right = node.borrow().right.clone();

            let left = node.borrow().left.clone();
            node.borrow_mut().left = None;
            node.borrow_mut().right = left;
        }
        root = node.borrow().right.clone();
    }
}
