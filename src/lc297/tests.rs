use super::Codec;
use crate::utils::binary_tree::btree;

#[test]
fn serialize() {
    let codec = Codec::new();
    assert_eq!(codec.serialize(btree![]), "[]");
    assert_eq!(codec.serialize(btree![1, 2]), "[1,2]");
    assert_eq!(codec.serialize(btree![1, null, 2]), "[1,null,2]");
    assert_eq!(
        codec.serialize(btree![1, null, 2, null, null, 3]),
        "[1,null,2,null,null,3]"
    );
    assert_eq!(
        codec.serialize(btree![1, 2, 3, null, null, 4, 5]),
        "[1,2,3,null,null,4,5]"
    );
}

#[test]
fn deserialize() {
    let codec = Codec::new();
    assert_eq!(codec.deserialize("[]".to_string()), btree![]);
    assert_eq!(codec.deserialize("[1,2]".to_string()), btree![1, 2]);
    assert_eq!(
        codec.deserialize("[1,null,2]".to_string()),
        btree![1, null, 2]
    );
    assert_eq!(
        codec.deserialize("[1,null,2,null,null,3]".to_string()),
        btree![1, null, 2, null, null, 3]
    );
    assert_eq!(
        codec.deserialize("[1,2,3,null,null,4,5]".to_string()),
        btree![1, 2, 3, null, null, 4, 5]
    );
}
