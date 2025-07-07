use super::MyLinkedList;

#[test]
fn forward() {
    let mut list = MyLinkedList::new();
    list.add_at_head(3);
    list.add_at_head(2);
    list.add_at_head(1);

    assert_eq!(list.into_iter().collect::<Vec<i32>>(), [1, 2, 3]);
}
