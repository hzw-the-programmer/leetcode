use super::*;
use crate::utils::binary_tree::*;

#[test]
fn test() {
    assert_eq!(min_depth(build!([3, 9, 20, null, null, 15, 7])), 2);
}
