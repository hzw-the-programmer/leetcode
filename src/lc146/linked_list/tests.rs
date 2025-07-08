use super::LinkedList;

#[test]
fn iter() {
    let mut list = LinkedList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    assert_eq!(list.iter().collect::<Vec<_>>(), [&1, &2, &3]);
    assert_eq!(list.iter().rev().collect::<Vec<_>>(), [&3, &2, &1]);
    assert_eq!((&list).into_iter().collect::<Vec<_>>(), [&1, &2, &3]);
}

#[test]
fn iter_mut() {
    let mut list = LinkedList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    list.iter_mut().for_each(|n| *n += 1);

    assert_eq!(
        list.iter_mut().collect::<Vec<_>>(),
        [&mut 2, &mut 3, &mut 4]
    );
    assert_eq!(
        list.iter_mut().rev().collect::<Vec<_>>(),
        [&mut 4, &mut 3, &mut 2]
    );
    assert_eq!(
        (&mut list).into_iter().collect::<Vec<_>>(),
        [&mut 2, &mut 3, &mut 4]
    );
}

#[test]
fn into_iter() {
    let mut list = LinkedList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    assert_eq!(list.clone().into_iter().collect::<Vec<_>>(), [1, 2, 3]);
    assert_eq!(list.into_iter().rev().collect::<Vec<_>>(), [3, 2, 1]);
}

#[test]
fn push_pop() {
    let mut list = LinkedList::new();

    for _ in 0..2 {
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), None);
    }
}

#[test]
fn collect() {
    let mut list = [1, 2, 3].iter().copied().collect::<LinkedList<_>>();
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn eq() {
    let list1 = [1, 2, 3].iter().copied().collect::<LinkedList<_>>();
    let list2 = [1, 2, 3].iter().copied().collect::<LinkedList<_>>();
    assert_eq!(list1, list2);

    let list3 = [3, 2, 1].iter().copied().collect::<LinkedList<_>>();
    assert_ne!(list1, list3);
}
