use super::*;
use crate::utils::binary_tree::btree;

#[test]
fn test() {
    assert_eq!(min_depth(btree![3, 9, 20, null, null, 15, 7]), 2);
}
