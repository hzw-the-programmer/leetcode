use super::reverse_k_group;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (list![1, 2, 3, 4, 5], 2, list![2, 1, 4, 3, 5]),
        (list![1, 2, 3, 4, 5], 3, list![3, 2, 1, 4, 5]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(reverse_k_group(test.0.clone(), test.1), test.2, "{}", i);
    }
}
