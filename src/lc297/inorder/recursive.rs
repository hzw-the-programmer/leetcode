use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub struct Codec {
    none: String,
    start: String,
    end: String,
}

impl Codec {
    pub fn new() -> Self {
        Self {
            none: "X".to_string(),
            start: "(".to_string(),
            end: ")".to_string(),
        }
    }

    pub fn with(none: String, start: String, end: String) -> Self {
        Self { none, start, end }
    }

    // time : O(n)
    // space: O(n)
    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        self.serialize_recursive(root.as_deref(), &mut res);
        res
    }

    // time : O(n)
    // space: O(n)
    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut data = &data[..];
        self.deserialize_recursive(&mut data).unwrap()
    }

    fn serialize_recursive(&self, root: Option<&RefCell<TreeNode>>, res: &mut String) {
        match root {
            None => {
                res.push_str(&self.none);
            }
            Some(node) => {
                res.push_str(&self.start);
                self.serialize_recursive(node.borrow().left.as_deref(), res);
                res.push_str(&self.end);

                res.push_str(&node.borrow().val.to_string());

                res.push_str(&self.start);
                self.serialize_recursive(node.borrow().right.as_deref(), res);
                res.push_str(&self.end);
            }
        }
    }

    fn deserialize_recursive(&self, data: &mut &str) -> Result<Option<Rc<RefCell<TreeNode>>>, u8> {
        if data.len() >= self.none.len() && data[..self.none.len()] == self.none {
            *data = &data[self.none.len()..];
            return Ok(None);
        }

        let left = self.subtree(data)?;
        let (val, len) = parse_i32(data.as_bytes())?;
        *data = &data[len..];
        let right = self.subtree(data)?;

        Ok(Some(Rc::new(RefCell::new(TreeNode { val, left, right }))))
    }

    fn subtree(&self, data: &mut &str) -> Result<Option<Rc<RefCell<TreeNode>>>, u8> {
        if data.len() < self.start.len() {
            return Err(1);
        }
        *data = &data[self.start.len()..];

        let node = self.deserialize_recursive(data)?;

        if data.len() < self.end.len() {
            return Err(2);
        }
        *data = &data[self.end.len()..];

        Ok(node)
    }
}

fn parse_i32(mut bytes: &[u8]) -> Result<(i32, usize), u8> {
    let (mut n, mut sign, mut len) = (0, 1, 0);

    if bytes.is_empty() {
        return Err(3);
    }

    if bytes[0] == b'-' {
        sign = -1;
        len += 1;
        bytes = &bytes[1..];
    }

    if bytes.is_empty() || to_digit(bytes[0]).is_none() {
        return Err(4);
    }

    for &b in bytes {
        if let Some(i) = to_digit(b) {
            n = n * 10 + i;
            len += 1;
        } else {
            break;
        }
    }

    Ok((n * sign, len))
}

fn to_digit(b: u8) -> Option<i32> {
    let i = b as i32 - b'0' as i32;
    if i >= 0 && i <= 10 { Some(i) } else { None }
}
