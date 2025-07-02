use super::is_palindrome;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 2, 1], true),
        (list![1, 2], false),
        (list![1], true),
        (list![1, 0, 1], true),
        (list![1, 1, 2, 1], false),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(is_palindrome(test.0.clone()), test.1, "{}", i);
    }
}
