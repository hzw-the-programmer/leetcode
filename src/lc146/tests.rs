use super::LRUCache;

#[test]
fn t1() {
    let mut cache = LRUCache::new(2);

    cache.put(1, 1);
    assert_eq!(cache.iter().copied().collect::<Vec<_>>(), [(1, 1)]);

    cache.put(2, 2);
    assert_eq!(cache.iter().copied().collect::<Vec<_>>(), [(2, 2), (1, 1)]);

    assert_eq!(cache.get(1), 1);
    assert_eq!(cache.iter().copied().collect::<Vec<_>>(), [(1, 1), (2, 2)]);

    cache.put(3, 3);
    assert_eq!(cache.iter().copied().collect::<Vec<_>>(), [(3, 3), (1, 1)]);

    assert_eq!(cache.get(2), -1);
    assert_eq!(cache.iter().copied().collect::<Vec<_>>(), [(3, 3), (1, 1)]);

    cache.put(4, 4);
    assert_eq!(cache.iter().copied().collect::<Vec<_>>(), [(4, 4), (3, 3)]);

    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.iter().copied().collect::<Vec<_>>(), [(4, 4), (3, 3)]);

    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.iter().copied().collect::<Vec<_>>(), [(3, 3), (4, 4)]);

    assert_eq!(cache.get(4), 4);
    assert_eq!(cache.iter().copied().collect::<Vec<_>>(), [(4, 4), (3, 3)]);
}
