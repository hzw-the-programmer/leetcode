use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test_1() {
    assert!(is_balanced(btree![3, 9, 20, null, null, 15, 7]));
    assert!(!is_balanced(btree![1, 2, 2, 3, 3, null, null, 4, 4]));
    assert!(is_balanced(btree![]));

    assert!(!is_balanced(btree![1, 2, 3, 4, null, null, 5, 6]));
}
