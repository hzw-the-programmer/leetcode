use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// time : O(n)
// space: O(1)
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
                (Some(left), Some(right)) => match delete_min(Some(right)) {
                    (right, Some(min)) => {
                        assert!(min.borrow().left.is_none());
                        assert!(min.borrow().right.is_none());
                        min.borrow_mut().left = Some(left);
                        min.borrow_mut().right = right;
                        Some(min)
                    }
                    _ => None,
                },
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

fn delete_min(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
    match root {
        None => (None, None),
        Some(root) => {
            let (left, right) = (root.borrow().left.clone(), root.borrow().right.clone());
            match (left, right) {
                (None, _) => {
                    let right = root.borrow_mut().right.take();
                    (right, Some(root))
                }
                (Some(left), _) => {
                    let mut parent = root.clone();
                    let mut left = left;
                    loop {
                        let node = left.borrow().left.clone();
                        match node {
                            None => break,
                            Some(node) => {
                                parent = left.clone();
                                left = node;
                            }
                        }
                    }
                    parent.borrow_mut().left = left.borrow_mut().right.take();
                    (Some(root), Some(left))
                }
            }
        }
    }
}

// pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
//     let (mut current, mut parent) = (root.clone(), None);

//     while let Some(node) = current.take() {
//         if key < node.borrow().val {
//             current = node.borrow().left.clone();
//             parent = Some(node);
//         } else if key > node.borrow().val {
//             current = node.borrow().right.clone();
//             parent = Some(node);
//         } else {
//             let mut bm = node.borrow_mut();
//             let new = match (bm.left.take(), bm.right.take()) {
//                 (None, None) => None,
//                 (Some(node), None) | (None, Some(node)) => Some(node),
//                 (Some(left), Some(right)) => {
//                     let min = find_min(right.clone());
//                     min.borrow_mut().left = Some(left);
//                     Some(right)
//                 }
//             };

//             match parent.take() {
//                 None => return new,
//                 Some(parent) => {
//                     let mut bm = parent.borrow_mut();
//                     match (bm.left.clone(), bm.right.clone()) {
//                         (Some(left), _) if Rc::ptr_eq(&left, &node) => bm.left = new,
//                         (_, Some(right)) if Rc::ptr_eq(&right, &node) => bm.right = new,
//                         _ => unreachable!(),
//                     }
//                 }
//             }
//         }
//     }

//     root
// }

// fn find_min(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
//     let mut current = root;
//     loop {
//         let node = current.borrow().left.clone();
//         match node {
//             None => break,
//             Some(node) => current = node,
//         }
//     }
//     current
// }
