use super::*;
use crate::binary_tree::*;

#[test]
fn test_1() {
    let root = new_tree(1, None, new_tree(2, new_tree(3, None, None), None));
    assert_eq!(preorder_traversal(root), vec![1, 2, 3]);
}

#[test]
fn test_2() {
    let root = new_tree(
        1,
        new_tree(
            2,
            new_tree(4, None, None),
            new_tree(5, new_tree(6, None, None), new_tree(7, None, None)),
        ),
        new_tree(3, None, new_tree(8, new_tree(9, None, None), None)),
    );
    assert_eq!(preorder_traversal(root), vec![1, 2, 4, 5, 6, 7, 3, 8, 9]);
}

#[test]
fn test_3() {
    let root = None;
    assert_eq!(preorder_traversal(root), vec![]);
}

#[test]
fn test_4() {
    let root = new_tree(1, None, None);
    assert_eq!(preorder_traversal(root), vec![1]);
}
