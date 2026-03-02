use super::num_buses_to_destination;
use crate::vec_2d;

#[test]
fn t1() {
    let tests = [
        (vec_2d![[1, 2, 7], [3, 6, 7]], 1, 6, 2),
        (
            vec_2d![[7, 12], [4, 5, 15], [6], [15, 19], [9, 12, 13]],
            15,
            12,
            -1,
        ),
        (vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6, 2),
        (vec![vec![1, 2, 3]], 2, 2, 0),
        (vec![vec![1, 2], vec![3, 4]], 1, 4, -1),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            num_buses_to_destination(test.0.clone(), test.1, test.2),
            test.3,
            "{}",
            i
        );
    }
}
