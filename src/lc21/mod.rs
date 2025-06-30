// 21. Merge Two Sorted Lists

use crate::utils::singly_linked_list::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(node),
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                node1.next = merge_two_lists(node1.next.take(), Some(node2));
                Some(node1)
            } else {
                node2.next = merge_two_lists(Some(node1), node2.next.take());
                Some(node2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge_two_lists;
    use crate::utils::singly_linked_list::list;

    #[test]
    fn t1() {
        let tests = [
            (list![1, 2, 4], list![1, 3, 4], list![1, 1, 2, 3, 4, 4]),
            (list![], list![], list![]),
            (list![], list![0], list![0]),
        ];

        for (i, test) in tests.iter().enumerate() {
            assert_eq!(
                merge_two_lists(test.0.clone(), test.1.clone()),
                test.2,
                "{}",
                i
            );
        }
    }
}
