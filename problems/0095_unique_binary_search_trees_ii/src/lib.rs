/// NO. 95: Unique Binary Search Trees II

pub struct Solution;
use leetcode::TreeNode;

// ----- submission codes start here -----

use std::cell::RefCell;
use std::rc::Rc;
type TreeList = Vec<Option<Rc<RefCell<TreeNode>>>>;

fn gen_trees(start: i32, end: i32) -> TreeList {
    if start > end {
        vec![None]
    } else {
        (start..=end)
            .into_iter()
            .flat_map(|idx| {
                let left_trees = gen_trees(start, idx - 1).into_iter();
                let right_trees = gen_trees(idx + 1, end).into_iter();
                left_trees.flat_map(move |l| {
                    right_trees
                        .clone()
                        .map(move |r| (idx, l.clone(), r.clone()))
                })
            })
            .fold(vec![], |mut trees, (idx, left, right)| {
                trees.push(Some(Rc::new(RefCell::new(TreeNode {
                    val: idx,
                    left,
                    right,
                }))));
                trees
            })
    }
}

impl Solution {
    pub fn generate_trees(n: i32) -> TreeList {
        if n == 0 {
            vec![]
        } else {
            gen_trees(1, n)
        }
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
