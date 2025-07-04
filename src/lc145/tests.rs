use super::*;
use crate::utils::binary_tree::btree;

#[test]
fn test() {
    assert_eq!(postorder_traversal(btree![1, null, 2, 3]), vec![3, 2, 1]);
    assert_eq!(
        postorder_traversal(btree![1, 2, 3, 4, 5, null, 8, null, null, 6, 7, 9]),
        vec![4, 6, 7, 5, 2, 9, 8, 3, 1]
    );
    assert_eq!(postorder_traversal(btree![]), vec![]);
    assert_eq!(postorder_traversal(btree![1]), vec![1]);

    assert_eq!(postorder_traversal(btree![1, 2, 2]), vec![2, 2, 1]);
}
