use crate::utils::singly_linked_list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn recursive(mut next: Box<ListNode>, cur: Box<ListNode>) -> Option<Box<ListNode>> {
        if next.next.is_none() {
            next.next = Some(cur);
            Some(next)
        } else {
            recursive(next.next.replace(cur).unwrap(), next)
        }
    }

    let mut cur = head?;
    if let Some(next) = cur.next.take() {
        recursive(next, cur)
    } else {
        Some(cur)
    }
}
