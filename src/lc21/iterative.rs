use crate::utils::singly_linked_list::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut list1, mut list2) = (list1, list2);
    let mut res = None;
    let mut current = &mut res;
    loop {
        match (list1, list2) {
            (None, None) => break,
            (Some(node), None) | (None, Some(node)) => {
                *current = Some(node);
                break;
            }
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    (list1, list2) = (node1.next.take(), Some(node2));
                    *current = Some(node1);
                } else {
                    (list1, list2) = (Some(node1), node2.next.take());
                    *current = Some(node2);
                }
                current = &mut (*current).as_mut().unwrap().next;
            }
        }
    }
    res
}
