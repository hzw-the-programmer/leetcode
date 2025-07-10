use crate::lc21::merge_two_lists;
use crate::utils::singly_linked_list::ListNode;

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut res = None;
    for list in lists {
        res = merge_two_lists(res, list);
    }
    res
}
