use super::merge;
use crate::utils::singly_linked_list::list;

#[test]
fn t1() {
    let tests = [
        (
            vec![1, 2, 3, 0, 0, 0],
            3,
            vec![2, 5, 6],
            3,
            vec![1, 2, 2, 3, 5, 6],
        ),
        (vec![1], 1, vec![], 0, vec![1]),
        (vec![0], 0, vec![1], 1, vec![1]),
    ];

    for (i, test) in tests.iter().enumerate() {
        let mut nums1 = test.0.clone();
        merge(&mut nums1, test.1, &test.2, test.3);
        assert_eq!(nums1, test.4, "{}", i);
    }
}
