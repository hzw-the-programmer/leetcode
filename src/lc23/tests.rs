use super::merge_k_lists;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (
            vec![list![1, 4, 5], list![1, 3, 4], list![2, 6]],
            list![1, 1, 2, 3, 4, 4, 5, 6],
        ),
        (vec![], list![]),
        (vec![list![]], list![]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(merge_k_lists(test.0.clone()), test.1, "{}", i);
    }
}
