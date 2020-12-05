use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stack = Vec::new();

        loop {
            if let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            } else if let Some(node) = stack.pop() {
                let node_ref = node.borrow();

                k -= 1;

                if k == 0 {
                    return node_ref.val;
                } else {
                    root = node_ref.right.clone();
                }
            } else {
                break;
            }
        }

        0
    }
}

impl super::Solution for Solution {
    fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::kth_smallest(root, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}