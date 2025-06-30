// 19. Remove Nth Node From End of List

use crate::utils::singly_linked_list::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: 0, next: head };

    let mut fast: *mut ListNode = &mut dummy;
    let mut slow: *mut ListNode = fast;

    unsafe {
        for _ in 0..n {
            if (*fast).next.is_some() {
                fast = &mut **(*fast).next.as_mut().unwrap();
            } else {
                return None;
            }
        }

        while (*fast).next.is_some() {
            fast = &mut **(*fast).next.as_mut().unwrap();
            slow = &mut **(*slow).next.as_mut().unwrap();
        }

        (*slow).next = (*slow).next.take().unwrap().next.take();
    }

    dummy.next
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
