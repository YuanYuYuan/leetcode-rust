/// NO. 98: Validate Binary Search Tree

pub struct Solution;
pub use leetcode::{vec_to_tree, TreeNode};

// ----- submission codes start here -----

use std::{cell::RefCell, rc::Rc};
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = Vec::new();
        let mut current = root;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }
            current = stack.pop();
            if let Some(node) = current {
                if let Some(p_node) = prev {
                    if node.borrow().val <= p_node.borrow().val {
                        return false;
                    }
                }
                prev = Some(node.clone());
                current = node.borrow().right.clone();
            }
        }
        true
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let tree1 = vec_to_tree(vec![Some(2), Some(1), Some(3)]);
        let tree2 = vec_to_tree(vec![Some(5), Some(1), None, None, Some(3), Some(6)]);
        assert_eq!(Solution::is_valid_bst(tree1), true);
        assert_eq!(Solution::is_valid_bst(tree2), false);
    }
}
