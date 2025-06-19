use super::super::parse_i32;
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
        let mut data = &data[self.start.len()..];
        self.deserialize_recursive(&mut data).unwrap()
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

    fn deserialize_recursive(&self, data: &mut &str) -> Result<Option<Rc<RefCell<TreeNode>>>, u8> {
        if data.len() >= self.delimiter.len() && data[..self.delimiter.len()] == self.delimiter {
            *data = &data[self.delimiter.len()..];
        }

        if data.len() >= self.none.len() && data[..self.none.len()] == self.none {
            *data = &data[self.none.len()..];
            Ok(None)
        } else {
            Ok(Some(Rc::new(RefCell::new(TreeNode {
                val: parse_i32(data)?,
                left: self.deserialize_recursive(data)?,
                right: self.deserialize_recursive(data)?,
            }))))
        }
    }
}
