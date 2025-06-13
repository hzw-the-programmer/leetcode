use super::can_finish;
use crate::utils::macros::vec_2d;

#[test]
fn test() {
    assert!(can_finish(2, vec_2d![[1, 0]]));
    assert!(!can_finish(2, vec_2d![[1, 0], [0, 1]]));
}
