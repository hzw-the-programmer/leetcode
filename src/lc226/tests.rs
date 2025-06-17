use super::invert_tree;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    assert_eq!(
        invert_tree(btree![4, 2, 7, 1, 3, 6, 9]),
        btree![4, 7, 2, 9, 6, 3, 1]
    );
    assert_eq!(invert_tree(btree![2, 1, 3]), btree![2, 3, 1]);
    assert_eq!(invert_tree(btree![]), btree![]);
}
