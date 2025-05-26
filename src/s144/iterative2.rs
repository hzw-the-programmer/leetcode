use crate::utils::binary_tree::*;

pub fn preorder_traversal(root: Tree) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![];
    let mut tree = root;
    while tree.is_some() || !stack.is_empty() {
        while let Some(node) = tree {
            res.push(node.borrow().val);
            tree = node.borrow_mut().left.take();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            tree = node.borrow_mut().right.take();
        }
    }
    res
}
