use crate::utils::binary_tree::*;

pub fn level_order(root: Tree) -> Vec<Vec<i32>> {
    let mut res = Vec::<Vec<i32>>::new();

    if let Some(node) = root {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(node);
        while !queue.is_empty() {
            let n = queue.len();
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                v.push(node.val);
                if node.left.is_some() {
                    queue.push_back(node.left.clone().unwrap());
                }
                if node.right.is_some() {
                    queue.push_back(node.right.clone().unwrap());
                }
            }
            res.push(v);
        }
    }

    res
}

// pub fn level_order(root: Tree) -> Vec<Vec<i32>> {
//     let mut res: Vec<Vec<i32>> = vec![];

//     let mut stack = vec![(0, root)];
//     while let Some((l, tree)) = stack.pop() {
//         if let Some(node) = tree {
//             let node = node.borrow();
//             if l < res.len() {
//                 res[l].push(node.val);
//             } else {
//                 res.push(vec![node.val]);
//             }
//             stack.push((l + 1, node.right.clone()));
//             stack.push((l + 1, node.left.clone()));
//         }
//     }

//     res
// }
