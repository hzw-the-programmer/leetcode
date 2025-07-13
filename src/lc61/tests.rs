use super::rotate_right;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 3, 4, 5], 2, list![4, 5, 1, 2, 3]),
        (list![0, 1, 2], 4, list![2, 0, 1]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(rotate_right(test.0.clone(), test.1), test.2, "{}", i);
    }
}
