use super::delete_duplicates;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 3, 3, 4, 4, 5], list![1, 2, 5]),
        (list![1, 1, 1, 2, 3], list![2, 3]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(delete_duplicates(test.0.clone()), test.1, "{}", i);
    }
}
