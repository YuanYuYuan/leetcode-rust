use std::{cell::RefCell, rc::Rc};


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn vec_to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(v) = vec[0] {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.as_ref().unwrap().clone());
        for children in vec[1..].chunks(2) {
            let mut iter = children.into_iter();
            let parent = queue.pop_front().unwrap();
            if let Some(Some(v)) = iter.next() {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if let Some(Some(v)) = iter.next() {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
        root
    } else {
        None
    }

}

#[macro_export]
macro_rules! tree {
    () => { None };
    ($( $e:expr ), *) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec: Vec<_> = vec
                .into_iter()
                .map(|x| x.parse::<i32>().ok()).collect();
            vec_to_tree(vec)
        }
    };
    ($( $e:expr, ) *) => { tree![$($e), *] };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_to_tree() {
        // [6, 3, 5, 1, 2, N, 4], N: None
        //
        //           6
        //          / \
        //         3   5
        //        / \   \
        //       1   2   4
        let node1 = Some(Rc::new(RefCell::new(
            TreeNode {val: 1, left: None, right: None},
        )));
        let node2 = Some(Rc::new(RefCell::new(
            TreeNode {val: 2, left: None, right: None},
        )));
        let node3 = Some(Rc::new(RefCell::new(
            TreeNode {val: 3, left: node1, right: node2},
        )));
        let node4 = Some(Rc::new(RefCell::new(
            TreeNode {val: 4, left: None, right: None},
        )));
        let node5 = Some(Rc::new(RefCell::new(
            TreeNode {val: 5, left: None, right: node4},
        )));
        let node6 = Some(Rc::new(RefCell::new(
            TreeNode {val: 6, left: node3, right: node5},
        )));
        assert_eq!(
            vec_to_tree(vec![
                Some(6),
                Some(3),
                Some(5),
                Some(1),
                Some(2),
                None,
                Some(4),
            ]),
            node6,
        );
    }

    #[test]
    fn tree_macro() {
        assert_eq!(
            vec_to_tree(vec![
                Some(1),
                None,
                Some(2),
                None,
            ]),
            tree![1, null, 2, null]
        );
    }
}
