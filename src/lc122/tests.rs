use super::max_profit;

#[test]
fn t1() {
    let tests = [
        (vec![7, 1, 5, 3, 6, 4], 7),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![7, 6, 4, 3, 1], 0),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(max_profit(test.0.clone()), test.1, "{}", i);
    }
}
