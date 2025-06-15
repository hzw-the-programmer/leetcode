use super::has_path_sum;
use crate::utils::binary_tree::btree;

#[test]
fn test() {
    assert!(has_path_sum(
        btree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
        22
    ));
    assert!(!has_path_sum(btree![1, 2, 3], 5));
    assert!(!has_path_sum(btree![], 0));
}
