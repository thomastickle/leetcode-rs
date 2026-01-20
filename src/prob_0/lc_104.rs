// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use crate::lc_lib::solution::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(n) => {
                    let left_depth = max_depth(&n.borrow().left);
                    let right_depth = max_depth(&n.borrow().right);
                    1 + left_depth.max(right_depth)
                }
                None => 0,
            }
        }

        max_depth(&root)
    }
}

#[cfg(test)]
mod tests {
    use crate::lc_lib::solution::Solution;
    use crate::prob_0::lc_104::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_max_depth() {
        let val1 = create_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = Solution::max_depth(val1);
        assert_eq!(result, 3);

        let val2 = create_tree(vec![Some(1), None, Some(2)]);
        let result = Solution::max_depth(val2);
        assert_eq!(result, 2);
    }

    fn create_tree(p0: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if p0.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(p0[0].unwrap())));
        let mut queue = vec![root.clone()];
        let mut i = 1;
        while i < p0.len() {
            let current = queue.remove(0);
            if let Some(val) = p0[i] {
                let left_child = Rc::new(RefCell::new(TreeNode::new(val)));
                current.borrow_mut().left = Some(left_child.clone());
                queue.push(left_child);
            }
            i += 1;
            if i < p0.len() {
                if let Some(val) = p0[i] {
                    let right_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    current.borrow_mut().right = Some(right_child.clone());
                    queue.push(right_child);
                }
                i += 1;
            }
        }
        Some(root)
    }
}
