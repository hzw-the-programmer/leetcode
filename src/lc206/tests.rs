use super::reverse_list;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 3, 4, 5], list![5, 4, 3, 2, 1]),
        (list![1, 2], list![2, 1]),
        (list![], list![]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(reverse_list(test.0.clone()), test.1, "{}", i);
    }
}
