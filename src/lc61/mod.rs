// 61. Rotate List

use crate::utils::singly_linked_list::ListNode;

pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut count = 0;
    let mut current = head.as_ref();
    while let Some(node) = current {
        count += 1;
        current = node.next.as_ref();
    }

    if count == 0 {
        return None;
    }

    let k = k % count;

    if k == 0 {
        return head;
    }

    let mut cur = head.as_mut().unwrap();
    for _ in 0..count - k - 1 {
        cur = cur.next.as_mut().unwrap();
    }

    let mut res = cur.next.take();

    let mut cur = res.as_mut().unwrap();
    for _ in 0..k - 1 {
        cur = cur.next.as_mut().unwrap();
    }
    cur.next = head;

    res
}

#[cfg(test)]
mod tests;
