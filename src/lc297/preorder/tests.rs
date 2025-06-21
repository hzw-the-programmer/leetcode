use super::Codec;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (btree![], "#"),
        (btree![1], "1,#,#"),
        (btree![1, 2], "1,2,#,#,#"),
        (btree![1, 2, 3], "1,2,#,#,3,#,#"),
        (btree![1, null, 2], "1,#,2,#,#"),
        (btree![1, null, 2, 3], "1,#,2,3,#,#,#"),
        (btree![1, 2, 3, null, null, 4, 5], "1,2,#,#,3,4,#,#,5,#,#"),
        (btree![9, 3, 2, 4, 1, null, 6], "9,3,4,#,#,1,#,#,2,#,6,#,#"),
        (
            btree![
                4, -7, -3, null, null, -9, -3, 9, -7, -4, null, 6, null, -6, -6, null, null, 0, 6,
                5, null, 9, null, null, -1, -4, null, null, null, -2
            ],
            "4,-7,#,#,-3,-9,9,6,0,#,-1,#,#,6,-4,#,#,#,#,-7,-6,5,#,#,#,-6,9,-2,#,#,#,#,-3,-4,#,#,#",
        ),
    ];

    let codec = Codec::new();

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(codec.serialize(test.0.clone()), test.1, "{} failed", i);
    }

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            codec.deserialize(test.1.to_string()),
            test.0,
            "{} failed",
            i
        );
    }
}

#[test]
fn t2() {
    let tree = btree![
        4, -7, -3, null, null, -9, -3, 9, -7, -4, null, 6, null, -6, -6, null, null, 0, 6, 5, null,
        9, null, null, -1, -4, null, null, null, -2
    ];

    let codec = Codec::new();
    let text = codec.serialize(tree.clone());
    assert_eq!(codec.deserialize(text), tree);
}

#[test]
#[should_panic]
fn e1() {
    let codec = Codec::new();
    codec.deserialize("1,#,2,abc,#,#,#".to_string());
}

#[test]
fn t3() {
    let tests = [
        ("", btree![]),
        ("1,,", btree![1]),
        ("1,2,,,", btree![1, 2]),
        ("1,2,,,3,,", btree![1, 2, 3]),
        ("1,,2,,", btree![1, null, 2]),
        ("1,,2,3,,,", btree![1, null, 2, 3]),
        ("1,2,,,3,4,,,5,,", btree![1, 2, 3, null, null, 4, 5]),
        ("9,3,4,,,1,,,2,,6,,", btree![9, 3, 2, 4, 1, null, 6]),
        (
            "4,-7,,,-3,-9,9,6,0,,-1,,,6,-4,,,,,-7,-6,5,,,,-6,9,-2,,,,,-3,-4,,,",
            btree![
                4, -7, -3, null, null, -9, -3, 9, -7, -4, null, 6, null, -6, -6, null, null, 0, 6,
                5, null, 9, null, null, -1, -4, null, null, null, -2
            ],
        ),
    ];
    let codec = Codec::new();
    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            codec.deserialize(test.0.to_string()),
            test.1,
            "{} failed",
            i
        );
    }
}
