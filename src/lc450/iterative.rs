use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let (mut current, mut parent) = (root.clone(), None);

    while let Some(node) = current.take() {
        if key < node.borrow().val {
            current = node.borrow().left.clone();
            parent = Some(node);
        } else if key > node.borrow().val {
            current = node.borrow().right.clone();
            parent = Some(node);
        } else {
            let mut bm = node.borrow_mut();
            let new = match (bm.left.take(), bm.right.take()) {
                (None, None) => None,
                (Some(node), None) | (None, Some(node)) => Some(node),
                (Some(left), Some(right)) => {
                    let min = find_min(right.clone());
                    min.borrow_mut().left = Some(left);
                    Some(right)
                }
            };

            match parent.take() {
                None => return new,
                Some(parent) => {
                    let mut bm = parent.borrow_mut();
                    match (bm.left.clone(), bm.right.clone()) {
                        (Some(left), _) if Rc::ptr_eq(&left, &node) => bm.left = new,
                        (_, Some(right)) if Rc::ptr_eq(&right, &node) => bm.right = new,
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    root
}

fn find_min(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let mut current = root;
    loop {
        let node = current.borrow().left.clone();
        match node {
            None => break,
            Some(node) => current = node,
        }
    }
    current
}
