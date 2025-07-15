// 92. Reverse Linked List II

use crate::utils::singly_linked_list::ListNode;

pub fn reverse_between(
    mut head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut begin = None;

    let mut i = 1;
    let mut cur = &mut head;
    while let Some(node) = (*cur).take() {
        if i == left {
            begin = Some(node);
            break;
        } else {
            (*cur) = Some(node);
            cur = &mut (*cur).as_mut().unwrap().next;
            i += 1;
        }
    }

    if begin.is_none() {
        return head;
    }

    let p: *mut Option<Box<ListNode>> = &mut begin.as_mut().unwrap().next;
    let mut prev = None;
    while let Some(mut node) = begin {
        begin = node.next.take();
        node.next = prev;
        prev = Some(node);
        if i == right {
            break;
        }
        i += 1;
    }

    (*cur) = prev;
    unsafe {
        *p = begin;
    }

    head
}

#[cfg(test)]
mod tests;
