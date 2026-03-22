use super::roman_to_int;

#[test]
fn t1() {
    let tests = [("III", 3), ("LVIII", 58), ("MCMXCIV", 1994)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(roman_to_int(test.0.to_string()), test.1, "{i}");
    }
}
