use crate::utils::binary_tree::*;

pub fn level_order(root: Tree) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];

    let mut stack = vec![(0, root)];
    while let Some((l, tree)) = stack.pop() {
        if let Some(node) = tree {
            let node = node.borrow();
            if l < res.len() {
                res[l].push(node.val);
            } else {
                res.push(vec![node.val]);
            }
            stack.push((l + 1, node.right.clone()));
            stack.push((l + 1, node.left.clone()));
        }
    }

    res
}
