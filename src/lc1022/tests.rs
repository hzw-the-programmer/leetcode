use super::sum_root_to_leaf;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    assert_eq!(sum_root_to_leaf(btree![1, 0, 1, 0, 1, 0, 1]), 22);
    assert_eq!(sum_root_to_leaf(btree![0]), 0);
}
