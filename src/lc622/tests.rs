use super::MyCircularQueue;

#[test]
fn t1() {
    let mut que = MyCircularQueue::new(3);
    assert!(que.en_queue(1));
    assert!(que.en_queue(2));
    assert!(que.en_queue(3));
    assert!(!que.en_queue(4));
    assert_eq!(que.rear(), 3);
    assert!(que.is_full());
    assert!(que.de_queue());
    assert!(que.en_queue(4));
    assert_eq!(que.rear(), 4);
}
