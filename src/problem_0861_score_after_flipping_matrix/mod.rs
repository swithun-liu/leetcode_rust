pub mod iterative;

pub trait Solution {
    fn matrix_score(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 0, 1, 1], [1, 0, 1, 0], [1, 1, 0, 0]] as &dyn Matrix<_>, 39),
            (&[[0]], 1),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::matrix_score(grid.to_vec()), expected);
        }
    }
}
