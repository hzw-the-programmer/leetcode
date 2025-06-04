use crate::utils::binary_tree::*;
use crate::utils::singly_linked_list::*;
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    recursive(head.as_deref(), None)
}

fn recursive(start: Option<&ListNode>, end: Option<&ListNode>) -> Option<Rc<RefCell<TreeNode>>> {
    let finish = |cursor: Option<&ListNode>| match (cursor, end) {
        (Some(f), Some(e)) if ptr::eq(f, e) => true,
        (None, None) => true,
        (Some(_), Some(_)) => false,
        (Some(_), None) => false,
        (None, Some(_)) => unreachable!(),
    };

    if finish(start) {
        return None;
    }

    let mut fast = start;
    let mut slow = start;
    while !finish(fast) {
        fast = fast.unwrap().next.as_deref();
        if !finish(fast) {
            fast = fast.unwrap().next.as_deref();
            slow = slow.unwrap().next.as_deref();
        }
    }

    Some(Rc::new(RefCell::new(TreeNode {
        val: slow.unwrap().val,
        left: recursive(start, slow),
        right: recursive(slow.unwrap().next.as_deref(), end),
    })))
}
