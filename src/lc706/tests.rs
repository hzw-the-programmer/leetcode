use super::MyHashMap;

#[test]
fn t1() {
    let mut map = MyHashMap::new();
    map.put(1, 1);
    map.put(2, 2);
    assert_eq!(map.get(1), 1);
    assert_eq!(map.get(3), -1);
    map.put(2, 1);
    assert_eq!(map.get(2), 1);
    map.remove(2);
    assert_eq!(map.get(2), -1);
}
