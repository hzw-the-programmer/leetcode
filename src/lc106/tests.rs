use super::*;
use crate::utils::binary_tree::btree;

#[test]
fn test() {
    assert_eq!(
        build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
        btree![3, 9, 20, null, null, 15, 7]
    );
    assert_eq!(build_tree(vec![-1], vec![-1]), btree![-1]);
}
