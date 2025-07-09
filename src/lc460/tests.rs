use super::LFUCache;

#[test]
fn t1() {
    let mut lfu = LFUCache::new(2);

    lfu.put(1, 1); // {1: [1=1]}
    lfu.dump();

    lfu.put(2, 2); // {1: [2=2,1=1]}
    lfu.dump();

    assert_eq!(lfu.get(1), 1); // {1: [2=2], 2: [1=1]}
    lfu.dump();

    lfu.put(3, 3); // {1: [3=3], 2: [1=1]}
    lfu.dump();

    assert_eq!(lfu.get(2), -1);

    assert_eq!(lfu.get(3), 3); // {2: [3=3, 1=1]}
    lfu.dump();

    lfu.put(4, 4); // {1: [4=4], 2: [3=3]}
    lfu.dump();

    assert_eq!(lfu.get(1), -1);

    assert_eq!(lfu.get(3), 3); // {1: [4=4], 3: [3=3]}
    lfu.dump();

    assert_eq!(lfu.get(4), 4); // {2: [4=4], 3: [3=3]}
    lfu.dump();
}
