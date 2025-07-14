// 82. Remove Duplicates from Sorted List II

use crate::utils::singly_linked_list::ListNode;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = None;
    let mut resp = &mut res;

    let mut cur = head;
    while let Some(mut node) = cur.take() {
        match node.next.take() {
            None => {
                *resp = Some(node);
            }
            Some(mut next) => {
                if node.val == next.val {
                    cur = next.next.take();
                    while let Some(mut n) = cur.take() {
                        if n.val == node.val {
                            cur = n.next.take();
                        } else {
                            cur = Some(n);
                            break;
                        }
                    }
                } else {
                    *resp = Some(node);
                    resp = &mut (*resp).as_mut().unwrap().next;
                    cur = Some(next);
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests;
