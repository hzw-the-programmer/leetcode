use crate::utils::singly_linked_list::ListNode;

pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(mut head) => {
            if val == head.val {
                head.next.take()
            } else {
                head.next = delete_node(head.next.take(), val);
                Some(head)
            }
        }
    }
}
