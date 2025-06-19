use super::Codec;
use crate::utils::binary_tree::btree;

#[test]
fn serialize() {
    let codec = Codec::new();
    assert_eq!(codec.serialize(btree![]), "X");
    assert_eq!(codec.serialize(btree![1]), "(X)1(X)");
    assert_eq!(codec.serialize(btree![1, 2]), "((X)2(X))1(X)");
    assert_eq!(codec.serialize(btree![1, 2, 3]), "((X)2(X))1((X)3(X))");

    assert_eq!(codec.serialize(btree![1, null, 2]), "(X)1((X)2(X))");
    assert_eq!(
        codec.serialize(btree![1, null, 2, 3]),
        "(X)1(((X)3(X))2(X))"
    );
    assert_eq!(
        codec.serialize(btree![1, 2, 3, null, null, 4, 5]),
        "((X)2(X))1(((X)4(X))3((X)5(X)))" // "1,2,#,#,3,4,#,#,5,#,#"
    );
    assert_eq!(
        codec.serialize(btree![9, 3, 2, 4, 1, null, 6]),
        "(((X)4(X))3((X)1(X)))9((X)2((X)6(X)))" // "9,3,4,#,#,1,#,#,2,#,6,#,#"
    );

    assert_eq!(
        codec.serialize(btree![
            4, -7, -3, null, null, -9, -3, 9, -7, -4, null, 6, null, -6, -6, null, null, 0, 6, 5,
            null, 9, null, null, -1, -4, null, null, null, -2
        ]),
        "((X)-7(X))4((((((X)0((X)-1(X)))6(((X)-4(X))6(X)))9(X))-9((((X)5(X))-6(X))-7((((X)-2(X))9(X))-6(X))))-3(((X)-4(X))-3(X)))" // "4,-7,#,#,-3,-9,9,6,0,#,-1,#,#,6,-4,#,#,#,#,-7,-6,5,#,#,#,-6,9,-2,#,#,#,#,-3,-4,#,#,#"
    );
}

#[test]
fn deserialize() {
    let codec = Codec::new();
    assert_eq!(codec.deserialize("X".to_string()), btree![]);
    assert_eq!(codec.deserialize("(X)1(X)".to_string()), btree![1]);
    assert_eq!(codec.deserialize("((X)2(X))1(X)".to_string()), btree![1, 2]);
    assert_eq!(
        codec.deserialize("((X)2(X))1((X)3(X))".to_string()),
        btree![1, 2, 3]
    );

    assert_eq!(
        codec.deserialize("(X)1((X)2(X))".to_string()),
        btree![1, null, 2]
    );
    assert_eq!(
        codec.deserialize("(X)1(((X)3(X))2(X))".to_string()),
        btree![1, null, 2, 3]
    );
    assert_eq!(
        codec.deserialize("((X)2(X))1(((X)4(X))3((X)5(X)))".to_string()),
        btree![1, 2, 3, null, null, 4, 5]
    );
    assert_eq!(
        codec.deserialize("(((X)4(X))3((X)1(X)))9((X)2((X)6(X)))".to_string()),
        btree![9, 3, 2, 4, 1, null, 6]
    );

    assert_eq!(
        codec.deserialize(
            "((X)-7(X))4((((((X)0((X)-1(X)))6(((X)-4(X))6(X)))9(X))-9((((X)5(X))-6(X))-7((((X)-2(X))9(X))-6(X))))-3(((X)-4(X))-3(X)))"
                .to_string()
        ),
        btree![
            4, -7, -3, null, null, -9, -3, 9, -7, -4, null, 6, null, -6, -6, null, null, 0, 6, 5,
            null, 9, null, null, -1, -4, null, null, null, -2
        ]
    );
}
