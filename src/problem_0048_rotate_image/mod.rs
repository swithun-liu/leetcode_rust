pub mod naive;
pub mod reverse_then_transpose;

pub trait Solution {
    fn rotate(matrix: &mut Vec<Vec<i32>>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[1, 2, 3] as &[_], &[4, 5, 6], &[7, 8, 9]] as &[&[_]],
                &[&[7, 4, 1] as &[_], &[8, 5, 2], &[9, 6, 3]] as &[&[_]],
            ),
            (
                &[&[5, 1, 9, 11], &[2, 4, 8, 10], &[13, 3, 6, 7], &[15, 14, 12, 16]],
                &[&[15, 13, 2, 5], &[14, 3, 4, 1], &[12, 6, 8, 9], &[16, 7, 10, 11]],
            ),
        ];

        for (matrix, expected) in test_cases.iter().copied() {
            let mut matrix = matrix.iter().map(|row| row.to_vec()).collect();

            S::rotate(&mut matrix);

            assert_eq!(matrix, expected);
        }
    }
}
