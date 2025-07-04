use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use core::num::ParseIntError;
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

        let mut stack = Vec::new();
        stack.push(root);
        while let Some(node) = stack.pop() {
            match node {
                None => {
                    res.push_str(&self.none);
                    res.push_str(&self.delimiter);
                }
                Some(node) => {
                    let node = node.borrow();
                    res.push_str(&node.val.to_string());
                    res.push_str(&self.delimiter);
                    stack.push(node.right.clone());
                    stack.push(node.left.clone());
                }
            }
        }
        res.truncate(res.len() - self.delimiter.len());

        res.push_str(&self.end);
        res
    }

    // time : O(n)
    // space: O(n)
    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let iter = data
            .trim_start_matches(&self.start)
            .trim_end_matches(&self.end)
            .split(&self.delimiter)
            .map(|s| match s {
                "#" | "" => Ok(None),
                _ => s.parse().map(|val| Some(val)),
            });

        Self::from_iter(iter).unwrap()
    }

    fn from_iter(
        mut iter: impl Iterator<Item = Result<Option<i32>, ParseIntError>>,
    ) -> Result<Option<Rc<RefCell<TreeNode>>>, ParseIntError> {
        let root = match iter.next() {
            None | Some(Ok(None)) => return Ok(None),
            Some(Ok(Some(val))) => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(Err(err)) => return Err(err),
        };

        let mut stack = vec![root.clone()];
        while let Some(top) = stack.pop() {
            if let Some(value) = iter.next() {
                if let Some(val) = value? {
                    let node = Rc::new(RefCell::new(TreeNode::new(val)));
                    if top.borrow().left.is_none() {
                        top.borrow_mut().left = Some(node.clone());
                        stack.push(top);
                        stack.push(node);
                    } else {
                        top.borrow_mut().right = Some(node.clone());
                        stack.push(node);
                    }
                } else if top.borrow().left.is_none() {
                    if let Some(value) = iter.next() {
                        if let Some(val) = value? {
                            let node = Rc::new(RefCell::new(TreeNode::new(val)));
                            top.borrow_mut().right = Some(node.clone());
                            stack.push(node);
                        }
                    }
                }
            }
        }

        Ok(Some(root))
    }
}
