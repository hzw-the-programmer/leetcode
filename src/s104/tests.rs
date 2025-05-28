use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test_1() {
    assert_eq!(max_depth(build!([3, 9, 20, null, null, 15, 7])), 3);
}

#[test]
fn test_2() {
    assert_eq!(max_depth(build!([1, null, 2])), 2);
}
