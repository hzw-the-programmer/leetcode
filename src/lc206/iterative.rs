use crate::utils::singly_linked_list::ListNode;

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = pre;
        pre = Some(node);
    }
    pre
}
