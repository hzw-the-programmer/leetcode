use super::*;

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut res = vec![];
    generate(&mut res, 1, n);
    res
}

fn generate(res: &mut Vec<Tree>, start: i32, end: i32) {
    if start > end {
        res.push(None);
        return;
    }

    for val in start..=end {
        let mut left_trees = vec![];
        let mut right_trees = vec![];

        generate(&mut left_trees, start, val - 1);
        generate(&mut right_trees, val + 1, end);

        for left in left_trees {
            for right in right_trees.iter() {
                res.push(new_tree(val, left.clone(), right.clone()));
            }
        }
    }
}
