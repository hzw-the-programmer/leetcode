use super::hamming_weight;

#[test]
fn test() {
    assert_eq!(hamming_weight(11), 3);
    assert_eq!(hamming_weight(128), 1);
    assert_eq!(hamming_weight(2147483645), 30);

    assert_eq!(hamming_weight(-1), 32);
}
