use crate::utils::binary_tree::*;

pub fn inorder_traversal(mut root: Tree) -> Vec<i32> {
    let mut res = vec![];

    while let Some(node) = root {
        if node.borrow().left.is_none() {
            res.push(node.borrow().val);
            root = node.borrow().right.clone();
        } else {
            let mut predecessor = node.borrow().left.clone().unwrap();
            while predecessor.borrow().right.is_some() {
                let right_node = predecessor.borrow().right.clone().unwrap();
                if std::rc::Rc::ptr_eq(&right_node, &node) {
                    break;
                }
                predecessor = right_node;
            }
            if predecessor.borrow().right.is_none() {
                predecessor.borrow_mut().right = Some(node.clone());
                root = node.borrow().left.clone();
            } else {
                predecessor.borrow_mut().right = None;
                res.push(node.borrow().val);
                root = node.borrow().right.clone();
            }
        }
    }

    res
}
