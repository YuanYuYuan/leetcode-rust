/// NO. 100: Same Tree

pub struct Solution;
pub use leetcode::{TreeNode, vec_to_tree, tree};

// ----- submission codes start here -----

use std::rc::Rc;
use std::cell::RefCell;
pub type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn stack_method(p: Tree, q: Tree) -> bool {
        let mut stack = vec![p, q];

        while let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
            match (a, b) {
                (None, None) => continue,
                (Some(a), Some(b)) => {
                    let (a, b) = (
                        Rc::try_unwrap(a).unwrap().into_inner(),
                        Rc::try_unwrap(b).unwrap().into_inner()
                    );
                    if a.val != b.val {
                        return false
                    } else {
                        stack.push(a.left);
                        stack.push(b.left);
                        stack.push(a.right);
                        stack.push(b.right);
                    }

                },
                _ => return false
            }
        }
        true
    }

    fn recursive_method(p: Tree, q: Tree) -> bool {
        fn recursive(p: &Tree, q: &Tree) -> bool {
            match (p, q) {
                (Some(p), Some(q)) => {
                    let (p, q) = (p.borrow(), q.borrow());
                    p.val == q.val &&
                        recursive(&p.left, &q.left) &&
                        recursive(&p.right, &q.right)
                }
                (None, None) => true,
                _ => false,
            }
        }

        recursive(&p, &q)
    }

    pub fn is_same_tree(p: Tree, q: Tree) -> bool {
        // Solution::recursive_method(p, q)
        Solution::stack_method(p, q)
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_same_tree(
            tree![1, 2, 3],
            tree![1, 2, 3]
        ), true);
        assert_eq!(Solution::is_same_tree(
            tree![1, 2, 3],
            tree![1, null, 2]
        ), false);
        assert_eq!(Solution::is_same_tree(
            tree![1, 2, 1],
            tree![1, 1, 2]
        ), false);
    }
}
