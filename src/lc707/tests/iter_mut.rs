use super::MyLinkedList;

#[test]
fn forward() {
    let mut list = MyLinkedList::new();

    list.add_at_tail(1);
    list.add_at_tail(2);
    list.add_at_tail(3);

    list.iter_mut().for_each(|n| *n += 1);

    assert_eq!(list.iter().copied().collect::<Vec<i32>>(), [2, 3, 4]);
}

#[test]
fn into_iter() {
    let mut list = MyLinkedList::new();

    list.add_at_tail(1);
    list.add_at_tail(2);
    list.add_at_tail(3);

    for n in &mut list {
        *n += 1;
    }

    assert_eq!(list.iter().copied().collect::<Vec<i32>>(), [2, 3, 4]);
}
