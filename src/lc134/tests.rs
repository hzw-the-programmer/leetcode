use super::can_complete_circuit;

#[test]
fn t1() {
    let tests = [
        (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2], 3),
        (vec![2, 3, 4], vec![3, 4, 3], -1),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            can_complete_circuit(test.0.clone(), test.1.clone()),
            test.2,
            "{i}"
        );
    }
}
