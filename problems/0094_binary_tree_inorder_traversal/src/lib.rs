/// NO. 94: Binary Tree Inorder Traversal

pub struct Solution;
use leetcode::{TreeNode, tree, vec_to_tree};

// ----- submission codes start here -----

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {

    fn recursive_method(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            if let Some(node) = root {
                let mut vec = recursive(&node.borrow().left);
                vec.push(node.borrow().val);
                vec.append(&mut recursive(&node.borrow().right));
                vec
            } else {
                vec![]
            }
        }
        recursive(&root)
    }

    fn iterative_method(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut stack = Vec::new();
        let mut curr = root;
        while curr.is_some() || !stack.is_empty() {
            while let Some(node) = curr {
                stack.push(node.clone());
                curr = node.borrow().left.clone();
            }
            curr = stack.pop();
            if let Some(node) = curr {
                vec.push(node.borrow().val);
                curr = node.borrow().right.clone();
            }
        }
        vec
    }

    fn morris_trasversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut curr = root;
        let mut out = Vec::new();

        while curr.is_some() {

            // return and move right if left is none
            if curr.as_ref().unwrap().borrow().left.is_none() {
                out.push(curr.as_ref().unwrap().borrow().val);
                curr = curr.unwrap().borrow().right.clone();
            } else {
                // find predecessor
                let mut pred = curr.as_ref().unwrap().borrow().left.clone();
                loop {
                    let pred_right = pred.as_ref().unwrap().borrow().right.clone();
                    if pred_right.is_none() || pred_right == curr {
                        break
                    } else {
                        pred = pred.unwrap().borrow().right.clone();
                    }
                }

                let pred_right = pred.as_ref().unwrap().borrow().right.clone();
                // link pred and go left
                if pred_right.is_none() {
                    pred.unwrap().borrow_mut().right = curr.clone();
                    curr = curr.unwrap().borrow().left.clone();
                }
                // return, remove pred link, and go back to pred
                else {
                    pred.unwrap().borrow_mut().right = None;
                    out.push(curr.as_ref().unwrap().borrow().val);
                    curr = curr.unwrap().borrow().right.clone();
                }
            }

        }

        out
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Solution::recursive_method(root)
        // Solution::iterative_method(root)
        Solution::morris_trasversal(root)
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            vec![1, 3, 2, 5, 4],
            Solution::inorder_traversal(tree![5, 3, 4, 1, 2]),
        );
    }
}
