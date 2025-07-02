// 234. Palindrome Linked List

use crate::utils::singly_linked_list::ListNode;

pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return true;
    }
    let mut fast: *mut ListNode = head.as_deref_mut().unwrap();
    let mut slow = fast;
    let mut count = 1;

    unsafe {
        while let Some(node) = (*fast).next.as_deref_mut() {
            fast = node;
            count += 1;
            if let Some(node) = (*fast).next.as_deref_mut() {
                fast = node;
                count += 1;
                slow = (*slow).next.as_deref_mut().unwrap();
            }
        }

        let head2 = (*slow).next.take();
        let mut pre = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = pre;
            pre = Some(node);
        }

        if count & 1 == 1 {
            pre = pre.as_mut().unwrap().next.take();
        }

        head2 == pre
    }
}

#[cfg(test)]
mod tests;
