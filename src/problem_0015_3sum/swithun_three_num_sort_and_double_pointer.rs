pub struct Solution;

/// sort and left & right pointer

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: HashSet<Vec<i32>> = HashSet::new();
        if nums.len() < 3 {
            return result.into_iter().collect();
        }

        let mut nums= nums.clone();
        nums.sort();

        for i in 0..(nums.len() - 2) {
            let mut L = i + 1;
            let mut R = nums.len() - 1;

            while L < R {
                let n_i = nums[i];
                let n_l = nums[L];
                let n_r = nums[R];
                let sum = n_i + n_l + n_r;
                if sum == 0 {
                    result.insert(vec![n_i, n_l, n_r]);
                    L += 1;
                    R -= 1;
                } else if sum > 0 {
                    R -= 1;
                } else {
                    L += 1;
                }
            }
        }

        result.into_iter().collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::three_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
