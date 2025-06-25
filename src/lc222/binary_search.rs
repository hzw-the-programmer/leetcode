use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     match root {
//         None => 0,
//         Some(root) => {
//             let mut level = 0;
//             let mut left = root.borrow().left.clone();
//             while let Some(node) = left {
//                 level += 1;
//                 left = node.borrow().left.clone();
//             }

//             let (mut low, mut high) = ((1 << level) - 1 + 1, (1 << level + 1) - 1);
//             while low < high {
//                 let mid = low + (high - low + 1) / 2;
//                 if exists(Some(root.clone()), level, mid) {
//                     low = mid;
//                 } else {
//                     high = mid - 1;
//                 }
//             }
//             low as _
//         }
//     }
// }

// fn exists(root: Option<Rc<RefCell<TreeNode>>>, level: usize, idx: usize) -> bool {
//     let mut bits = (1 << level) >> 1;

//     let mut current = root;
//     while bits != 0 && current.is_some() {
//         let node = current.unwrap();
//         current = if idx & bits == 0 {
//             node.borrow().left.clone()
//         } else {
//             node.borrow().right.clone()
//         };
//         bits >>= 1;
//     }

//     current.is_some()
// }

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(root) => {
            let mut level = 0;
            // let mut current = root.clone();
            // while current.borrow().left.is_some() {
            //     level += 1;
            //     let left = current.borrow().left.clone().unwrap();
            //     current = left;
            // }
            let mut left = root.borrow().left.clone();
            while let Some(node) = left {
                level += 1;
                left = node.borrow().left.clone();
            }

            let (mut low, mut high) = (0, 1 << level);
            while low < high {
                let mid = low + (high - low) / 2;
                if exists(Some(root.clone()), level, mid) {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            ((1 << level) - 1 + low) as _
        }
    }
}

fn exists(root: Option<Rc<RefCell<TreeNode>>>, level: usize, idx: usize) -> bool {
    let mut bits = 1 << level >> 1;

    let mut current = root;
    while bits != 0 && current.is_some() {
        let node = current.unwrap();
        current = if idx & bits == 0 {
            node.borrow().left.clone()
        } else {
            node.borrow().right.clone()
        };
        bits >>= 1;
    }

    current.is_some()
}
