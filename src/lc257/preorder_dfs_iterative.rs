use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut res = vec![];
    match root {
        None => {}
        Some(node) => {
            let mut stack = vec![(node, String::new())];
            while let Some((node, mut path)) = stack.pop() {
                let mut current = Some(node);
                while let Some(node) = current.take() {
                    let node = node.borrow();
                    path.push_str(&node.val.to_string());
                    match (node.left.clone(), node.right.clone()) {
                        (None, right) => {
                            if let Some(node) = right {
                                path.push_str("->");
                                stack.push((node, path));
                            } else {
                                res.push(path);
                            }
                            break;
                        }
                        (Some(left), right) => {
                            path.push_str("->");
                            if let Some(node) = right {
                                stack.push((node, path.clone()));
                            }
                            current = Some(left);
                        }
                    }
                }
            }
        }
    }
    res
}

// pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
//     let mut res = vec![];
//     let mut temp = String::new();
//     match root {
//         None => {}
//         Some(node) => {
//             let mut stack = vec![(node, 0)];
//             while let Some((node, len)) = stack.pop() {
//                 temp.truncate(len);
//                 let mut current = Some(node);
//                 while let Some(node) = current.take() {
//                     let node = node.borrow();
//                     temp.push_str(&node.val.to_string());
//                     match (node.left.clone(), node.right.clone()) {
//                         (None, None) => res.push(temp.clone()),
//                         (left, right) => {
//                             temp.push_str("->");
//                             current = left;
//                             if let Some(node) = right {
//                                 stack.push((node, temp.len()));
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     res
// }
