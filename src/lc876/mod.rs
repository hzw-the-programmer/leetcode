// 876. Middle of the Linked List

use crate::utils::singly_linked_list::ListNode;

pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast: *mut ListNode = head.as_deref_mut()?;
    let mut slow = fast;

    unsafe {
        if (*fast).next.is_none() {
            return head;
        }
        while let Some(node) = (*fast).next.as_deref_mut() {
            fast = node;
            if let Some(node) = (*fast).next.as_deref_mut() {
                fast = node;
                if (*fast).next.is_some() {
                    slow = (*slow).next.as_deref_mut().unwrap();
                }
            }
        }
        (*slow).next.take()
    }
}

#[cfg(test)]
mod tests;
