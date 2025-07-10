use crate::lc21::merge_two_lists;
use crate::utils::singly_linked_list::ListNode;

// time : O(kn * logn)
// space: O(logn)
pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.len() == 0 {
        return None;
    }

    while lists.len() > 1 {
        let len = lists.len();
        let mut merged = Vec::with_capacity((len + 1) >> 1);

        for i in (1..len).step_by(2) {
            merged.push(merge_two_lists(lists[i - 1].take(), lists[i].take()));
        }

        if len & 1 != 0 {
            merged.push(lists[len - 1].take());
        }

        lists = merged;
    }

    lists[0].take()
}
