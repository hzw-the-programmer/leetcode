use super::trap;

#[test]
fn t1() {
    let tests = [
        (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
        (vec![4, 2, 0, 3, 2, 5], 9),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(trap(test.0.clone()), test.1, "{i}");
    }
}
