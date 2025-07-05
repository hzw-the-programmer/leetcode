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
    assert_eq!(list.len(), 2);
    list.add_at_tail(4);
    assert_eq!(list.len(), 3);

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

#[test]
fn iter() {
    let mut list = MyLinkedList::new();
    list.add_at_head(3);
    list.add_at_head(2);
    list.add_at_head(1);

    let mut v = vec![];
    for &n in &list {
        v.push(n);
    }
    assert_eq!(v, [1, 2, 3]);
}

#[test]
fn iter_mut() {
    let mut list = MyLinkedList::new();
    list.add_at_tail(1);
    list.add_at_tail(2);
    list.add_at_tail(3);

    for n in &mut list {
        *n += 1;
    }
    assert_eq!(list.iter().copied().collect::<Vec<i32>>(), [2, 3, 4]);
}
