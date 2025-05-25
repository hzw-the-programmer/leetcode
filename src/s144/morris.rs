use crate::binary_tree::*;

pub fn preorder_traversal(mut root: Tree) -> Vec<i32> {
    let mut res = vec![];

    while let Some(node) = root {
        if let Some(mut predecessor) = node.borrow().left.clone() {
            while predecessor.borrow().right.is_some() {
                let right_node = predecessor.borrow().right.clone().unwrap();
                if std::rc::Rc::ptr_eq(&right_node, &node) {
                    break;
                }
                predecessor = right_node;
            }

            if predecessor.borrow().right.is_none() {
                res.push(node.borrow().val);
                root = node.borrow().left.clone();
                predecessor.borrow_mut().right = Some(node.clone());
            } else {
                predecessor.borrow_mut().right = None;
                root = node.borrow().right.clone();
            }
        } else {
            res.push(node.borrow().val);
            root = node.borrow().right.clone();
        }
    }

    res
}
