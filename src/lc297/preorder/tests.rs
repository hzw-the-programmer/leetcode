use super::Codec;
use crate::utils::binary_tree::btree;

#[test]
fn serialize() {
    let codec = Codec::new();
    assert_eq!(codec.serialize(btree![]), "#");
    assert_eq!(codec.serialize(btree![1]), "1,#,#");
    assert_eq!(codec.serialize(btree![1, 2]), "1,2,#,#,#");
    assert_eq!(codec.serialize(btree![1, 2, 3]), "1,2,#,#,3,#,#");

    assert_eq!(codec.serialize(btree![1, null, 2]), "1,#,2,#,#");
    assert_eq!(codec.serialize(btree![1, null, 2, 3]), "1,#,2,3,#,#,#");
    assert_eq!(
        codec.serialize(btree![1, 2, 3, null, null, 4, 5]),
        "1,2,#,#,3,4,#,#,5,#,#"
    );
    assert_eq!(
        codec.serialize(btree![9, 3, 2, 4, 1, null, 6]),
        "9,3,4,#,#,1,#,#,2,#,6,#,#"
    );
}

#[test]
fn deserialize() {
    let codec = Codec::new();
    assert_eq!(codec.deserialize("#".to_string()), btree![]);
    assert_eq!(codec.deserialize("1,#,#".to_string()), btree![1]);
    assert_eq!(codec.deserialize("1,2,#,#,#".to_string()), btree![1, 2]);
    assert_eq!(
        codec.deserialize("1,2,#,#,3,#,#".to_string()),
        btree![1, 2, 3]
    );

    assert_eq!(
        codec.deserialize("1,#,2,#,#".to_string()),
        btree![1, null, 2]
    );
    assert_eq!(
        codec.deserialize("1,#,2,3,#,#,#".to_string()),
        btree![1, null, 2, 3]
    );
    assert_eq!(
        codec.deserialize("1,2,#,#,3,4,#,#,5,#,#".to_string()),
        btree![1, 2, 3, null, null, 4, 5]
    );
    assert_eq!(
        codec.deserialize("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()),
        btree![9, 3, 2, 4, 1, null, 6]
    );
    assert_eq!(
        codec.deserialize(
            "4,-7,#,#,-3,-9,9,6,0,#,-1,#,#,6,-4,#,#,#,#,-7,-6,5,#,#,#,-6,9,-2,#,#,#,#,-3,-4,#,#,#"
                .to_string()
        ),
        btree![
            4, -7, -3, null, null, -9, -3, 9, -7, -4, null, 6, null, -6, -6, null, null, 0, 6, 5,
            null, 9, null, null, -1, -4, null, null, null, -2
        ]
    );
}

#[test]
fn t1() {
    let tree = btree![
        4, -7, -3, null, null, -9, -3, 9, -7, -4, null, 6, null, -6, -6, null, null, 0, 6, 5, null,
        9, null, null, -1, -4, null, null, null, -2
    ];

    let codec = Codec::new();
    let text = codec.serialize(tree.clone());
    assert_eq!(codec.deserialize(text), tree);
}
