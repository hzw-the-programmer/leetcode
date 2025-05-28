use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test() {
    assert_eq!(
        zigzag_level_order(build!([3, 9, 20, null, null, 15, 7])),
        vec![vec![3], vec![20, 9], vec![15, 7]]
    );
    assert_eq!(zigzag_level_order(build!([1])), vec![vec![1]]);
    assert_eq!(zigzag_level_order(build!([])), Vec::<Vec<i32>>::new());
}
