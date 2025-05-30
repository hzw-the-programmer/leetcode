use crate::utils::binary_tree::*;

pub fn preorder_traversal(root: Tree) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![];
    stack.push(root);
    while let Some(mut tree) = stack.pop() {
        while let Some(node) = tree {
            let mut node = node.borrow_mut();
            res.push(node.val);
            stack.push(node.right.take());
            tree = node.left.take();
        }
    }
    res
}

// pub fn preorder_traversal(root: Tree) -> Vec<i32> {
//     let mut res = vec![];
//     let mut stack = vec![];
//     stack.push(root);
//     while let Some(mut tree) = stack.pop() {
//         while let Some(node) = tree {
//             let node = node.borrow();
//             res.push(node.val);
//             stack.push(node.right.clone());
//             tree = node.left.clone();
//         }
//     }
//     res
// }
