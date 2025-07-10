use core::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use crate::utils::singly_linked_list::ListNode;

// time : O(L * logk)
// space: O(k)
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut res = None;
    let mut current = &mut res;

    let mut heap = lists
        .into_iter()
        .map(|node| Reverse(node))
        .collect::<BinaryHeap<_>>();

    while let Some(head) = heap.pop() {
        if let Reverse(Some(mut head)) = head {
            if head.next.is_some() {
                heap.push(Reverse(head.next.take()));
            }
            *current = Some(head);
            current = &mut (*current).as_mut().unwrap().next;
        }
    }

    res
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
