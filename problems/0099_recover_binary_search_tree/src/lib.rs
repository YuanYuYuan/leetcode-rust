/// NO. 99: Recover Binary Search Tree

pub struct Solution;
use leetcode::{TreeNode, tree, vec_to_tree};

// ----- submission codes start here -----

use std::rc::Rc;
use std::cell::RefCell;
type Tree = Option<Rc<RefCell<TreeNode>>>;

// find the 1st & 2nd numbers of wrong order and update prev by curr
pub fn check(curr: &Tree, first: &mut Tree, second: &mut Tree, prev: &mut Tree) {
    if let Some(curr) = curr {
        if let Some(prev) = prev {
            if prev.borrow().val >= curr.borrow().val {
                if first.is_none() {
                    *first = Some(prev.clone());
                }
                if first.is_some() {
                    *second = Some(curr.clone());
                }
            }
        }
        *prev = Some(curr.clone());
    }
}

impl Solution {

    // Time: O(n), Space: O(n)
    fn recursive_method(root: &mut Tree) {
        fn recursive(root: &Tree, first: &mut Tree, second: &mut Tree, prev: &mut Tree) {
            if let Some(root_) = root {
                recursive(&root_.borrow().left, first, second, prev);
                check(root, first, second, prev);
                recursive(&root_.borrow().right, first, second, prev);
            }
        }

        let (mut first, mut second, mut prev) = (None, None, None);
        recursive(&root, &mut first, &mut second, &mut prev);
        if let (Some(first), Some(second)) = (first, second) {
            std::mem::swap(
                &mut first.borrow_mut().val,
                &mut second.borrow_mut().val
            );
        }
    }

    // Time: O(n), Space: O(1)
    fn morris_trasveral(root: &mut Tree) {
        let mut curr = root.clone();
        let (mut first, mut second, mut prev) = (None, None, None);

        while curr.is_some() {
            if curr.as_ref().unwrap().borrow().left.is_none() {
                check(&curr, &mut first, &mut second, &mut prev);
                curr = curr.unwrap().borrow().right.clone();
            } else {
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
                if pred_right.is_none() {
                    pred.unwrap().borrow_mut().right = curr.clone();
                    curr = curr.unwrap().borrow().left.clone();
                } else {
                    check(&curr, &mut first, &mut second, &mut prev);
                    pred.unwrap().borrow_mut().right = None;
                    curr = curr.unwrap().borrow().right.clone();
                }
            }
        }

        if let (Some(first), Some(second)) = (first, second) {
            std::mem::swap(
                &mut first.borrow_mut().val,
                &mut second.borrow_mut().val
            );
        }
    }

    pub fn recover_tree(root: &mut Tree) {
        // Solution::recursive_method(root);
        Solution::morris_trasveral(root);
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut tree = tree![1, 3, null, null, 2];
        let correct = tree![3, 1, null, null, 2];
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, correct);
    }

    #[test]
    fn test2() {
        let mut tree = tree![3, 1, 4, null, null, 2];
        let correct = tree![2, 1, 4, null, null, 3];
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, correct);
    }
}
