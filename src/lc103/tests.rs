use super::*;
use crate::utils::binary_tree::btree;

#[test]
fn test() {
    assert_eq!(
        zigzag_level_order(btree![3, 9, 20, null, null, 15, 7]),
        vec![vec![3], vec![20, 9], vec![15, 7]]
    );
    assert_eq!(zigzag_level_order(btree![1]), vec![vec![1]]);
    assert_eq!(zigzag_level_order(btree![]), Vec::<Vec<i32>>::new());
}
