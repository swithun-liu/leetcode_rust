pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut start = matrix[0][0];
        let mut end = matrix[n - 1][n - 1];

        while start != end {
            let middle = start + (end - start) / 2;
            let mut rank = 0;
            let mut prev = n;

            for row in &matrix {
                prev = row[..prev].partition_point(|&v| v <= middle);

                if prev == 0 {
                    break;
                }

                rank += prev as i32;
            }

            if rank < k {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        start
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::kth_smallest(matrix, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
