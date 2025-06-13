use super::find_order;
use crate::utils::macros::vec_2d;

#[test]
fn test() {
    assert_eq!(find_order(2, vec_2d![[1, 0]]), vec![0, 1]);
    assert_eq!(
        find_order(4, vec_2d![[1, 0], [2, 0], [3, 1], [3, 2]]),
        vec![0, 1, 2, 3]
    );
    assert_eq!(find_order(1, vec_2d![]), vec![0]);
}
