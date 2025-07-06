use super::MyLinkedList;

#[test]
fn forward() {
    let mut list = MyLinkedList::new();
    list.add_at_head(3);
    list.add_at_head(2);
    list.add_at_head(1);

    assert_eq!(list.into_iter().collect::<Vec<i32>>(), [1, 2, 3]);
}

#[test]
fn reverse() {
    let mut list = MyLinkedList::new();
    list.add_at_head(3);
    list.add_at_head(2);
    list.add_at_head(1);

    assert_eq!(list.into_iter().rev().collect::<Vec<i32>>(), [3, 2, 1]);
}

#[test]
fn next_and_back() {
    let mut list = MyLinkedList::new();

    list.add_at_tail(1);
    list.add_at_tail(2);
    list.add_at_tail(3);
    list.add_at_tail(4);
    list.add_at_tail(5);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next_back(), Some(5));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), Some(4));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), None);
}
