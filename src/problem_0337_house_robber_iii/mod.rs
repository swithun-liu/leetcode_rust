use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(3), Some(2), Some(3), None, Some(3), None, Some(1)] as &[_], 7),
            (&[Some(3), Some(4), Some(5), Some(1), Some(3), None, Some(1)], 9),
        ];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(S::rob(test_utilities::make_tree(root.iter().copied())), expected);
        }
    }
}