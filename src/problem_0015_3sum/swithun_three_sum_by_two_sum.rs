pub struct Solution;

/// 3sum by 2sum
/// 1000ms  https://leetcode.cn/submissions/detail/321506668/

// ------------------------------------------------------ snip ------------------------------------------------------ //
use std::collections::{HashMap, HashSet};

impl Solution {
    // three numbers sum to zero
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut nums = nums.clone();
        nums.sort();
        let mut handle_num_value = nums[0];
        let mut handle_num_number = 0;

        let mut new_nums: Vec<i32> = Vec::new();
        for num in nums {
            if num != handle_num_value {
                handle_num_value = num;
                handle_num_number = 1;
            }
            if handle_num_value == 0 {
                if handle_num_number < 3 {
                    new_nums.insert(0, num);
                    handle_num_number += 1;
                }
            } else {
                if handle_num_number < 2 {
                    new_nums.insert(0, num);
                    handle_num_number += 1;
                }
            }
        }

        let mut result_value_set: HashSet<Vec<i32>> = HashSet::new();
        let mut handled_value: HashSet<i32> = HashSet::new();
        let mut working_pre_value = new_nums[0];
        for pos in 0..new_nums.len() {
            if pos > 0 {
                if new_nums[pos] == working_pre_value {
                    continue
                } else {
                    working_pre_value = new_nums[pos]
                }
            }
            handled_value.insert(new_nums[pos]);
            let two_sum_res = Self::two_sum(&new_nums, -new_nums[pos]);
            for temp_res in two_sum_res {
                let mut temp_set: HashSet<usize> = HashSet::new();
                temp_set.insert(pos);
                if temp_res.len() == 2 {
                    temp_set.insert(temp_res[0]);
                    temp_set.insert(temp_res[1]);

                    if temp_set.len() == 3 {
                        let mut value_vec: Vec<i32> = vec![];
                        for temp in temp_set {
                            value_vec.insert(0, new_nums[temp]);
                        }
                        value_vec.sort();
                        result_value_set.insert(value_vec);
                    }
                }
            }
        }
        result_value_set.into_iter().collect()
    }
    // tow sum
    fn two_sum(sorted_nums: &Vec<i32>, target_num: i32) -> Vec<Vec<usize>>{
        let mut pos_vec_set: HashSet<Vec<usize>> = HashSet::new();
        let mut cache_map: HashMap<i32, Vec<usize>> = HashMap::new();

        for pos1 in 0..sorted_nums.len() {
            match cache_map.get(&sorted_nums[pos1]) {
                None => {
                    cache_map.insert(sorted_nums[pos1], vec![pos1]);
                }
                Some(old_vec) => {
                    let mut new_vec: Vec<usize> = vec![pos1];
                    for num in old_vec {
                        new_vec.insert(0, *num);
                    }
                    cache_map.insert(sorted_nums[pos1], new_vec);
                }
            }
            let target = target_num - sorted_nums[pos1];
            match cache_map.get(&target) {
                None => {
                }
                Some(vec) => {
                    for ptr_pos2 in vec {
                        pos_vec_set.insert(vec![pos1, *ptr_pos2]);
                    }
                }
            }
        }
        pos_vec_set.into_iter().collect()
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
        print!("test_solution...");
        super::super::tests::run::<super::Solution>();
    }
}