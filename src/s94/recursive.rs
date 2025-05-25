use crate::binary_tree::*;

pub fn inorder_traversal(root: Tree) -> Vec<i32> {
    let mut res = Vec::new();
    inorder_traversal_recursive(&root, &mut res);
    res
}

fn inorder_traversal_recursive(root: &Tree, res: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder_traversal_recursive(&node.left, res);
        res.push(node.val);
        inorder_traversal_recursive(&node.right, res);
    }
}
