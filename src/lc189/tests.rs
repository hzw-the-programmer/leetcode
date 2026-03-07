use super::rotate;

#[test]
fn t1() {
    let tests = [
        (vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]),
        (vec![-1, -100, 3, 99], 2, vec![3, 99, -1, -100]),
    ];

    for (i, test) in tests.iter().enumerate() {
        let mut nums = test.0.clone();
        rotate(&mut nums, test.1);
        assert_eq!(nums, test.2, "{i}");
    }
}
