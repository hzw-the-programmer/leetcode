// 141. Linked List Cycle

use crate::utils::singly_linked_list::ListNode;
use core::ptr;

pub fn has_cycle(head: Option<&ListNode>) -> bool {
    let mut fast = head;
    let mut slow = head;
    while let Some(node) = fast {
        fast = node.next.as_deref();
        if let Some(node) = fast {
            fast = node.next.as_deref();
            slow = slow.unwrap().next.as_deref();

            if ptr::eq(node, slow.unwrap()) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests;
