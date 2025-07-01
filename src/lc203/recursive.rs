use crate::utils::singly_linked_list::ListNode;

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(mut node) => {
            if node.val == val {
                remove_elements(node.next.take(), val)
            } else {
                node.next = remove_elements(node.next.take(), val);
                Some(node)
            }
        }
    }
}
