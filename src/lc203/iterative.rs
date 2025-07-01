use crate::utils::singly_linked_list::ListNode;

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: 0, next: head };
    let mut cur = &mut dummy;
    while let Some(node) = cur.next.as_mut() {
        if node.val == val {
            cur.next = node.next.take();
        } else {
            cur = cur.next.as_deref_mut().unwrap();
        }
    }
    dummy.next
}
