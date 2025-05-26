use crate::utils::binary_tree::*;

pub fn preorder_traversal(root: Tree) -> Vec<i32> {
    let mut res = vec![];
    preorder_traversal_recursive(&root, &mut res);
    res
}

fn preorder_traversal_recursive(root: &Tree, res: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        res.push(node.val);
        preorder_traversal_recursive(&node.left, res);
        preorder_traversal_recursive(&node.right, res);
    }
}
