// 237. Delete Node in a Linked List

use crate::utils::singly_linked_list::ListNode;

pub fn delete_node(node: Option<&mut Box<ListNode>>) {
    match node {
        None => {}
        Some(node) => match node.next.take() {
            None => {}
            Some(mut next) => {
                node.val = next.val;
                node.next = next.next.take();
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::delete_node;
    use crate::utils::singly_linked_list::{ListNode, list};

    #[test]
    fn t1() {
        let tests = [
            (list![4, 5, 1, 9], 5, list![4, 1, 9]),
            (list![4, 5, 1, 9], 1, list![4, 5, 9]),
        ];

        for (i, test) in tests.iter().enumerate() {
            let mut head = test.0.clone();
            let node = find_mut(head.as_mut(), test.1);
            delete_node(node);
            assert_eq!(head, test.2, "{}", i);
        }
    }

    fn find_mut(root: Option<&mut Box<ListNode>>, val: i32) -> Option<&mut Box<ListNode>> {
        let mut current = root;
        while let Some(node) = current {
            if node.val == val {
                return Some(node);
            }
            current = node.next.as_mut();
        }
        None
    }
}
