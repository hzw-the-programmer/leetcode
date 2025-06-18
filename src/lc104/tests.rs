use super::*;
use crate::utils::binary_tree::btree;

#[test]
fn test_1() {
    assert_eq!(max_depth(btree![3, 9, 20, null, null, 15, 7]), 3);
}

#[test]
fn test_2() {
    assert_eq!(max_depth(btree![1, null, 2]), 2);
}

#[test]
fn test_3() {
    assert_eq!(max_depth(btree![1, 2, 3, 4, null, null, 5]), 3);
}
