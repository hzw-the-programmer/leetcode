// 24. Swap Nodes in Pairs

use crate::utils::singly_linked_list::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = None;
    let mut resp = &mut res;

    let mut cur = head;
    while let Some(mut node) = cur.take() {
        if let Some(mut node2) = node.next.take() {
            cur = node2.next.take();
            node2.next = Some(node);
            *resp = Some(node2);
            resp = &mut (*resp).as_mut().unwrap().next.as_mut().unwrap().next;
        } else {
            *resp = Some(node);
            break;
        }
    }

    res
}

#[cfg(test)]
mod tests;
