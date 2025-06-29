// LCR 136. 删除链表的节点

use crate::utils::singly_linked_list::ListNode;

pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(mut head) => {
            if val == head.val {
                head.next.take()
            } else {
                head.next = delete_node(head.next.take(), val);
                Some(head)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::delete_node;
    use crate::utils::singly_linked_list::list;

    #[test]
    fn t1() {
        let tests = [
            (list![4, 5, 1, 9], 5, list![4, 1, 9]),
            (list![4, 5, 1, 9], 1, list![4, 5, 9]),
        ];

        for (i, test) in tests.iter().enumerate() {
            assert_eq!(delete_node(test.0.clone(), test.1), test.2, "{}", i);
        }
    }
}
