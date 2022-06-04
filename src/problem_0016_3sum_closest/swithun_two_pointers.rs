pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut nums= nums;
        nums.sort();
        let mut diff: Option<i32> = None;
        for p_i in 0..(nums.len() - 2) {
            let mut p_l = p_i + 1;
            let mut p_r = nums.len() - 1;
            while p_l < p_r {
                let new_diff = nums[p_i] + nums[p_l] + nums[p_r] - target;
                if (diff == None) || (new_diff.abs() < i32::abs(diff.unwrap())) {
                    diff = Some(new_diff);
                }
                if new_diff < 0 {
                    p_l += 1;
                } else if new_diff > 0 {
                    p_r -= 1;
                } else {
                    return target;
                }
            }
        }
        diff.unwrap() + target
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        Self::three_sum_closest(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
