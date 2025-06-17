use super::merge_trees;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    assert_eq!(
        merge_trees(btree![1, 3, 2, 5], btree![2, 1, 3, null, 4, null, 7]),
        btree![3, 4, 5, 5, 4, null, 7]
    );
    assert_eq!(merge_trees(btree![1], btree![1, 2]), btree![2, 2]);
}
