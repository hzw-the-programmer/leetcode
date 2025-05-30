use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test() {
    assert_eq!(
        balance_bst(btree![
            1, null, 2, null, null, null, 3, null, null, null, null, null, null, null, 4
        ]),
        btree![3, 2, 4, 1, null]
    );
    assert_eq!(balance_bst(btree![2, 1, 3]), btree![2, 1, 3]);
}
