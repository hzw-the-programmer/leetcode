use crate::utils::binary_tree::*;
use crate::utils::singly_linked_list::*;
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

// time  : O(n*logn)
// space : O(logn)
pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    recursive(head.as_deref(), None)
}

fn recursive(left: Option<&ListNode>, right: Option<&ListNode>) -> Option<Rc<RefCell<TreeNode>>> {
    if finish(left, right) {
        return None;
    }

    let mut fast = left;
    let mut slow = left;
    while !finish(fast, right) {
        fast = fast.unwrap().next.as_deref();
        if !finish(fast, right) {
            fast = fast.unwrap().next.as_deref();
            slow = slow.unwrap().next.as_deref();
        }
    }

    Some(Rc::new(RefCell::new(TreeNode {
        val: slow.unwrap().val,
        left: recursive(left, slow),
        right: recursive(slow.unwrap().next.as_deref(), right),
    })))
}

fn finish(left: Option<&ListNode>, right: Option<&ListNode>) -> bool {
    match (left, right) {
        (Some(l), Some(r)) if ptr::eq(l, r) => true,
        (None, None) => true,
        (Some(_), Some(_)) => false,
        (Some(_), None) => false,
        (None, Some(_)) => unreachable!(),
    }
}
