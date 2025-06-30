use crate::utils::singly_linked_list::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(node),
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                node1.next = merge_two_lists(node1.next.take(), Some(node2));
                Some(node1)
            } else {
                node2.next = merge_two_lists(Some(node1), node2.next.take());
                Some(node2)
            }
        }
    }
}
