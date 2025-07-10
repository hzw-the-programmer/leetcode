use crate::lc21::merge_two_lists;
use crate::utils::singly_linked_list::ListNode;

// time : O(L * logk)
// space: O(logk)
pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    divide_merge(&mut lists)
}

fn divide_merge(lists: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
    if lists.len() == 0 {
        return None;
    } else if lists.len() == 1 {
        return lists[0].take();
    }

    let mid = lists.len() >> 1;
    merge_two_lists(
        divide_merge(&mut lists[..mid]),
        divide_merge(&mut lists[mid..]),
    )
}

// pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
//     if lists.len() == 0 {
//         return None;
//     } else if lists.len() == 1 {
//         return lists[0].take();
//     }

//     let mid = lists.len() >> 1;
//     let lists2 = lists.split_off(mid);
//     merge_two_lists(merge_k_lists(lists), merge_k_lists(lists2))
// }
