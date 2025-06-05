use crate::utils::{binary_tree::*, singly_linked_list::*};
use std::cell::RefCell;
use std::rc::Rc;

pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut head = head.as_deref();
    let n = len(head);
    inorder(&mut head, 0, n)
}

fn len(mut head: Option<&ListNode>) -> usize {
    let mut n = 0;
    while let Some(node) = head {
        n += 1;
        head = node.next.as_deref();
    }
    n
}

fn inorder(
    head: &mut Option<&ListNode>,
    left: usize,
    right: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    if left >= right {
        return None;
    }

    let mid = left + (right - left) / 2;
    // let mid = left + (right - left - 1) / 2;
    let mut node = TreeNode::new(0);
    node.left = inorder(head, left, mid);
    node.val = head.unwrap().val;
    *head = head.unwrap().next.as_deref();
    node.right = inorder(head, mid + 1, right);
    Some(Rc::new(RefCell::new(node)))
}
