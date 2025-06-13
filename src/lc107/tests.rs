use super::*;
use crate::utils::binary_tree::*;
use crate::utils::macros::vec_2d;

#[test]
fn test() {
    assert_eq!(
        level_order_bottom(build!([3, 9, 20, null, null, 15, 7])),
        vec_2d![[15, 7], [9, 20], [3]]
    );
    assert_eq!(level_order_bottom(build!([1])), vec_2d![[1]]);
    assert_eq!(level_order_bottom(build!([])), Vec::<Vec<i32>>::new());
}
