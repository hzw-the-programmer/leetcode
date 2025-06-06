use crate::utils::binary_tree::*;

pub fn inorder_traversal(root: Tree) -> Vec<i32> {
    let mut res = Vec::new();

    let mut stack = vec![];
    let mut current = root.clone();
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            current = node.borrow().left.clone();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            res.push(node.borrow().val);
            current = node.borrow().right.clone();
        }
    }

    res
}
