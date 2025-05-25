use crate::binary_tree::*;

pub fn preorder_traversal(mut root: Tree) -> Vec<i32> {
    let mut res = vec![];
    while let Some(root_node) = root.clone() {
        if let Some(mut node) = root_node.borrow().left.clone() {
            while node.borrow().right.is_some()
                && !std::rc::Rc::ptr_eq(
                    node.borrow().right.as_ref().unwrap(),
                    root.as_ref().unwrap(),
                )
            {
                let t = node.borrow().right.clone();
                node = t.unwrap();
            }

            if node.borrow().right.is_none() {
                res.push(root_node.borrow().val);
                node.borrow_mut().right = root.clone();
                root = root_node.borrow().left.clone();
            } else if std::rc::Rc::ptr_eq(
                node.borrow().right.as_ref().unwrap(),
                root.as_ref().unwrap(),
            ) {
                node.borrow_mut().right.take();
                root = root_node.borrow().right.clone();
            }
        } else {
            res.push(root_node.borrow().val);
            root = root_node.borrow().right.clone();
        }
    }
    res
}
