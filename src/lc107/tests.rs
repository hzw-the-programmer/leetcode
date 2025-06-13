use super::*;
use crate::utils::binary_tree::*;
use crate::utils::macros::nested_vec;

#[test]
fn test() {
    assert_eq!(
        level_order_bottom(build!([3, 9, 20, null, null, 15, 7])),
        nested_vec![[15, 7], [9, 20], [3]]
    );
    assert_eq!(level_order_bottom(build!([1])), nested_vec![[1]]);
    assert_eq!(level_order_bottom(build!([])), Vec::<Vec<i32>>::new());
}
