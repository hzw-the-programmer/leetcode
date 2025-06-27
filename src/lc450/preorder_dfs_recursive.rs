use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(root) => {
            let mut node = root.borrow_mut();
            if key < node.val {
                node.left = delete_node(node.left.take(), key);
                Some(root.clone())
            } else if key > node.val {
                node.right = delete_node(node.right.take(), key);
                Some(root.clone())
            } else {
                match (node.left.take(), node.right.take()) {
                    (None, None) => None,
                    (Some(node), None) | (None, Some(node)) => Some(node),
                    (Some(left), Some(right)) => {
                        let min = find_min(right.clone());
                        min.borrow_mut().left = Some(left);
                        Some(right)
                    }
                }
            }
        }
    }
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

// pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
//     match root {
//         None => None,
//         Some(root) => {
//             {
//                 let mut root = root.borrow_mut();
//                 if key == root.val {
//                     if root.right.is_none() {
//                         return root.left.take();
//                     } else {
//                         let right = root.right.clone().unwrap();
//                         if right.borrow().left.is_none() {
//                             root.val = right.borrow().val;
//                             root.right = right.borrow_mut().right.take();
//                         } else {
//                             let mut parent = right.clone();
//                             let mut left = right.borrow().left.clone().unwrap();
//                             while left.borrow().left.is_some() {
//                                 parent = left.clone();
//                                 let t = left.borrow().left.clone().unwrap();
//                                 left = t;
//                             }
//                             root.val = left.borrow().val;
//                             parent.borrow_mut().left = left.borrow_mut().right.take();
//                         }
//                     }
//                 } else if key < root.val {
//                     root.left = delete_node(root.left.take(), key);
//                 } else {
//                     root.right = delete_node(root.right.take(), key);
//                 }
//             }
//             Some(root)
//         }
//     }
// }
