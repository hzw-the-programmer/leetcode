use super::*;

#[test]
fn test_inorder_traversal() {
    let root = new_tree(1, None, new_tree(2, new_tree(3, None, None), None));
    assert_eq!(inorder_traversal(root), vec![1, 3, 2]);
}

#[test]
fn test_inorder_traversal_2() {
    let root = None;
    assert_eq!(inorder_traversal(root), vec![]);
}

#[test]
fn test_inorder_traversal_3() {
    let root = new_tree(1, None, None);
    assert_eq!(inorder_traversal(root), vec![1]);
}
