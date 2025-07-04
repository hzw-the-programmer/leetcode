use super::MyHashSet;

#[test]
fn t1() {
    let mut set = MyHashSet::new();
    set.add(1);
    set.add(2);
    assert!(set.contains(1));
    assert!(!set.contains(3));
    set.add(2);
    assert!(set.contains(2));
    set.remove(2);
    assert!(!set.contains(2));
}
