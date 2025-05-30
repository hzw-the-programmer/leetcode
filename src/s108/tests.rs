use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test() {
    assert_eq!(
        sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
        btree![0, -10, 5, null, -3, null, 9]
    );
    assert_eq!(sorted_array_to_bst(vec![1, 3]), btree![1, null, 3]);
    assert_eq!(
        sorted_array_to_bst_2(vec![-10, -3, 0, 5, 9]),
        btree![0, -3, 9, -10, null, 5]
    );
    assert_eq!(sorted_array_to_bst_2(vec![1, 3]), btree![3, 1]);
}
