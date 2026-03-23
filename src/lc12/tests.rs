use super::int_to_roman;

#[test]
fn t1() {
    let tests = [
        (3749, "MMMDCCXLIX"),
        (58, "LVIII"),
        (1994, "MCMXCIV"),
        (300, "CCC"),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(int_to_roman(test.0), test.1, "{i}");
    }
}
