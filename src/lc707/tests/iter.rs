use super::MyLinkedList;

#[test]
fn forward() {
    let mut list = MyLinkedList::new();
    list.add_at_head(3);
    list.add_at_head(2);
    list.add_at_head(1);

    assert_eq!(list.iter().copied().collect::<Vec<i32>>(), [1, 2, 3]);
}

#[test]
fn into_iter() {
    let mut list = MyLinkedList::new();
    list.add_at_tail(1);
    list.add_at_tail(2);
    list.add_at_tail(3);

    let mut v = vec![];
    for &n in &list {
        v.push(n);
    }
    assert_eq!(v, [1, 2, 3]);
}
