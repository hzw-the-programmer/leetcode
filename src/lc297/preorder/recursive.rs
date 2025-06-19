use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub struct Codec {
    none: String,
    delimiter: String,
    start: String,
    end: String,
}

impl Codec {
    pub fn new() -> Self {
        Self {
            none: "#".to_string(),
            delimiter: ",".to_string(),
            start: String::new(),
            end: String::new(),
        }
    }

    pub fn with(none: String, delimiter: String, start: String, end: String) -> Self {
        Self {
            none,
            delimiter,
            start,
            end,
        }
    }

    // time : O(n)
    // space: O(n)
    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        res.push_str(&self.start);
        self.serialize_recursive(root.as_deref(), &mut res);
        res.truncate(res.len() - self.delimiter.len());
        res.push_str(&self.end);
        res
    }

    // time : O(n)
    // space: O(n)
    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = data
            .trim_start_matches(&self.start)
            .trim_end_matches(&self.end)
            .split(&self.delimiter)
            .map(|s| match s {
                "#" => None,
                _ => s.parse().ok(),
            });

        Self::deserialize_recursive(&mut iter)
    }

    fn serialize_recursive(&self, root: Option<&RefCell<TreeNode>>, res: &mut String) {
        match root {
            None => {
                res.push_str(&self.none);
                res.push_str(&self.delimiter);
            }
            Some(node) => {
                res.push_str(&node.borrow().val.to_string());
                res.push_str(&self.delimiter);
                self.serialize_recursive(node.borrow().left.as_deref(), res);
                self.serialize_recursive(node.borrow().right.as_deref(), res);
            }
        }
    }

    fn deserialize_recursive(
        iter: &mut impl Iterator<Item = Option<i32>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match iter.next() {
            None | Some(None) => None,
            Some(Some(val)) => Some(Rc::new(RefCell::new(TreeNode {
                val: val,
                left: Self::deserialize_recursive(iter),
                right: Self::deserialize_recursive(iter),
            }))),
        }
    }
}
