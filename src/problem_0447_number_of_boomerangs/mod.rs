pub mod group_by_circles;

pub trait Solution {
    fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 0], [1, 0], [2, 0]] as &[_], 2),
            (&[[1, 1], [2, 2], [3, 3]], 2),
            (&[[1, 1]], 0),
            (&[[0, 0], [1, 0], [-1, 0], [0, 1], [0, -1]], 20),
        ];

        for (points, expected) in test_cases {
            assert_eq!(
                S::number_of_boomerangs(points.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
