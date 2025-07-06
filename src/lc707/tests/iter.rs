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
fn reverse() {
    let mut list = MyLinkedList::new();
    list.add_at_head(3);
    list.add_at_head(2);
    list.add_at_head(1);

    assert_eq!(list.iter().rev().copied().collect::<Vec<i32>>(), [3, 2, 1]);
}

#[test]
fn ref_into_iter() {
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

#[test]
fn next_and_back() {
    let mut list = MyLinkedList::new();

    list.add_at_tail(1);
    list.add_at_tail(2);
    list.add_at_tail(3);
    list.add_at_tail(4);
    list.add_at_tail(5);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next_back(), Some(&5));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next_back(), Some(&4));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next_back(), None);
}
