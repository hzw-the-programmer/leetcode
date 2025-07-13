// 24. Swap Nodes in Pairs

use crate::utils::singly_linked_list::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = None;
    let mut resp = &mut res;

    let mut cur = head;
    let mut prev = None;
    while cur.is_some() {
        if let Some(mut node) = cur.take() {
            if let Some(mut node2) = node.next.take() {
                cur = node2.next.take();
                node.next = prev;
                node2.next = Some(node);
                prev = Some(node2);
            } else {
                *resp = Some(node);
                return res;
            }
        }
        *resp = prev.take();
        resp = &mut (*resp).as_mut().unwrap().next.as_mut().unwrap().next;
    }

    res
}

#[cfg(test)]
mod tests;
