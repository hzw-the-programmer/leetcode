use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match (root, p, q) {
        (Some(root), Some(p), Some(q)) => {
            let (val, p_val, q_val) = (root.borrow().val, p.borrow().val, q.borrow().val);
            if val == p_val || val == q_val {
                Some(root)
            } else {
                let left = lowest_common_ancestor(
                    root.borrow().left.clone(),
                    Some(p.clone()),
                    Some(q.clone()),
                );
                let right = lowest_common_ancestor(root.borrow().right.clone(), Some(p), Some(q));
                match (left, right) {
                    (Some(_), Some(_)) => Some(root),
                    (Some(node), None) | (None, Some(node)) => Some(node),
                    _ => None,
                }
            }
        }
        _ => None,
    }
}

// pub fn lowest_common_ancestor(
//     root: Option<Rc<RefCell<TreeNode>>>,
//     p: Option<Rc<RefCell<TreeNode>>>,
//     q: Option<Rc<RefCell<TreeNode>>>,
// ) -> Option<Rc<RefCell<TreeNode>>> {
//     let (p, q) = (p.unwrap().borrow().val, q.unwrap().borrow().val);
//     let (mut path, mut paths) = (vec![], vec![]);

//     recursive(root, p, q, &mut path, &mut paths);

//     let len = paths[0].len().min(paths[1].len());
//     for i in 0..len {
//         if paths[0][i].borrow().val != paths[1][i].borrow().val {
//             return Some(paths[0][i - 1].clone());
//         }
//     }
//     Some(paths[0][len - 1].clone())
// }

// fn recursive(
//     root: Option<Rc<RefCell<TreeNode>>>,
//     p: i32,
//     q: i32,
//     path: &mut Vec<Rc<RefCell<TreeNode>>>,
//     paths: &mut Vec<Vec<Rc<RefCell<TreeNode>>>>,
// ) {
//     match root {
//         None => {}
//         Some(node) => {
//             path.push(node.clone());

//             let val = node.borrow().val;
//             if val == p || val == q {
//                 paths.push(path.clone());
//             }

//             if paths.len() < 2 {
//                 recursive(node.borrow().left.clone(), p, q, path, paths);
//                 if paths.len() < 2 {
//                     recursive(node.borrow().right.clone(), p, q, path, paths);
//                 }
//             }
//             path.pop();
//         }
//     }
// }
