// use super::has_cycle;
use super::has_cycle;
use crate::utils::singly_linked_list::{ListNode, list};
use core::{mem, ptr};

#[test]
fn t1() {
    let tests = [
        (list![3, 2, 0, -4], 1, true),
        (list![1, 2], 0, true),
        (list![1], -1, false),
    ];

    for (i, test) in tests.iter().enumerate() {
        let mut head = test.0.clone();
        link(head.as_deref_mut(), test.1);
        assert_eq!(has_cycle(head.as_deref()), test.2, "{}", i);
        unlink(head.as_deref_mut(), test.1);
    }
}

fn link(head: Option<&mut ListNode>, pos: i32) {
    if pos < 0 {
        return;
    }

    let mut cur = head;
    let mut i = 0;
    let mut raw = ptr::null_mut();

    while let Some(node) = cur {
        if i == pos {
            raw = node;
        }

        if node.next.is_none() {
            node.next = unsafe { Some(Box::from_raw(raw)) };
            break;
        }

        cur = node.next.as_deref_mut();
        i += 1;
    }
}

fn unlink(head: Option<&mut ListNode>, pos: i32) {
    if pos < 0 {
        return;
    }
    let mut cur = head;
    let mut i = 0;
    let mut ptr = ptr::null();

    while let Some(node) = cur {
        if i == pos {
            ptr = node;
        }

        if ptr::eq(node.next.as_deref().unwrap(), ptr) {
            mem::forget(node.next.take());
            break;
        }

        cur = node.next.as_deref_mut();
        i += 1;
    }
}
