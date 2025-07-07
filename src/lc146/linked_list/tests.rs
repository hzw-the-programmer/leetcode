use super::LinkedList;

#[test]
fn iter() {
    let mut list = LinkedList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    assert_eq!(list.iter().copied().collect::<Vec<_>>(), [1, 2, 3]);
}
