use super::MyLinkedList;

#[test]
fn reverse() {
    let mut list = MyLinkedList::new();

    list.add_at_tail(1);
    list.add_at_tail(2);
    list.add_at_tail(3);

    list.iter_mut().rev().for_each(|n| *n += 1);

    assert_eq!(list.iter().copied().collect::<Vec<i32>>(), [2, 3, 4]);
}

#[test]
fn next_and_back() {
    let mut list = MyLinkedList::new();

    list.add_at_tail(1);
    list.add_at_tail(2);
    list.add_at_tail(3);
    list.add_at_tail(4);
    list.add_at_tail(5);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next_back(), Some(&mut 5));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next_back(), Some(&mut 4));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next_back(), None);
}
