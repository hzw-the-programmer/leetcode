use super::MyCircularDeque;

#[test]
fn t1() {
    let mut deque = MyCircularDeque::new(3);
    assert!(deque.insert_last(1));
    assert!(deque.insert_last(2));
    assert!(deque.insert_front(3));
    assert!(!deque.insert_front(4));
    assert_eq!(deque.get_rear(), 2);
    assert!(deque.is_full());
    assert!(deque.delete_last());
    assert!(deque.insert_front(4));
    assert_eq!(deque.get_front(), 4);
}

#[test]
fn t2() {
    let mut deque = MyCircularDeque::new(4);
    assert!(deque.insert_front(9));
    assert!(deque.delete_last());
    assert_eq!(deque.get_rear(), -1);
    assert_eq!(deque.get_front(), -1);
    assert_eq!(deque.get_front(), -1);
    assert!(!deque.delete_front());
    assert!(deque.insert_front(6));
    assert!(deque.insert_last(5));
    assert!(deque.insert_front(9));
    assert_eq!(deque.get_front(), 9);
    assert!(deque.insert_front(6));
}

#[test]
fn t3() {
    let mut deque = MyCircularDeque::new(4);
    assert!(deque.insert_front(1));
    assert!(deque.insert_front(2));
    assert!(deque.delete_front());
    assert!(deque.delete_last());
}

#[test]
fn t4() {
    let mut deque = MyCircularDeque::new(3);
    assert!(deque.insert_last(1));
    assert!(deque.insert_last(2));
    assert!(deque.insert_front(3));
    assert!(!deque.insert_front(4));
    assert_eq!(deque.get_rear(), 2);
    assert!(deque.is_full());
    assert!(deque.delete_last());
    assert!(deque.insert_front(4));
    assert_eq!(deque.get_front(), 4);
}
