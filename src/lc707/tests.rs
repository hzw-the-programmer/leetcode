use super::MyLinkedList;

#[test]
fn t1() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(3);
    list.add_at_index(1, 2);
    assert_eq!(list.get(1), 2);
    list.delete_at_index(1);
    assert_eq!(list.get(1), 3);
}

#[test]
fn t2() {
    let mut list = MyLinkedList::new();
    list.add_at_head(7);
    list.add_at_head(2);
    list.add_at_head(1);

    list.add_at_index(3, 0);
    assert_eq!(list.get(3), 0);

    list.delete_at_index(2);
    assert_eq!(list.get(2), 0);

    list.add_at_head(6);

    list.add_at_tail(4);

    assert_eq!(list.get(4), 4);

    list.add_at_head(4);
    list.add_at_index(5, 0);
    list.add_at_head(6);
}

#[test]
fn t3() {
    let mut list = MyLinkedList::new();
    list.add_at_head(7);
    list.add_at_head(2);
    list.add_at_head(1);

    list.delete_at_index(2);
    list.add_at_tail(4);

    assert_eq!(list.get(2), 4);
}

#[test]
fn t4() {
    let mut list = MyLinkedList::new();
    list.add_at_head(3);
    list.add_at_head(2);
    list.add_at_head(1);

    list.delete_at_index(2);
    list.add_at_index(2, 3);
    list.add_at_tail(4);
    assert_eq!(list.get(3), 4);
}

#[test]
fn t5() {
    let mut list = MyLinkedList::new();
    list.add_at_head(3);
    list.add_at_head(2);
    list.add_at_head(1);

    list.delete_at_index(3);
}
