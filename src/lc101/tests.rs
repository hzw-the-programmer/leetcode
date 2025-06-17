use super::is_symmetric;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    assert!(is_symmetric(btree![1, 2, 2, 3, 4, 4, 3]));
    assert!(!is_symmetric(btree![1, 2, 2, null, 3, null, 3]));
}
