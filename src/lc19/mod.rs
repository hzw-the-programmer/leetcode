// 19. Remove Nth Node From End of List

use crate::utils::singly_linked_list::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
    if n < 1 {
        return None;
    }

    let mut fast = head.as_ref();
    while n > 0 {
        if let Some(node) = fast {
            fast = node.next.as_ref();
            n -= 1;
        } else {
            return None;
        }
    }

    if fast.is_none() {
        return head.unwrap().next.take();
    }

    let mut fast = fast.unwrap();
    let mut slow = head.as_ref().unwrap();
    while fast.next.is_some() {
        fast = fast.next.as_ref().unwrap();
        slow = slow.next.as_ref().unwrap();
    }

    let slow = &**slow as *const ListNode as *mut ListNode;
    unsafe {
        let mut next = (*slow).next.take().unwrap();
        (*slow).next = next.next.take();
    }

    head
}

#[cfg(test)]
mod tests {
    use super::remove_nth_from_end;
    use crate::utils::singly_linked_list::list;

    #[test]
    fn t1() {
        let tests = [
            (list![1, 2, 3, 4, 5], 2, list![1, 2, 3, 5]),
            (list![1], 1, list![]),
            (list![1, 2], 1, list![1]),
        ];

        for (i, test) in tests.iter().enumerate() {
            assert_eq!(remove_nth_from_end(test.0.clone(), test.1), test.2, "{}", i);
        }
    }
}
