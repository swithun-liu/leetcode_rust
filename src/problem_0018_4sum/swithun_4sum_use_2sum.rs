pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::{collections::{HashMap, HashSet}};


impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let nums = Self::trim_nums(&nums);
        if nums.len() < 4 {
            return vec![];
        }
        let mut all_results = vec![];
        Self::processor(&nums, 0, 4, target, HashSet::new(), &mut all_results);
        Self::map_to_num(&nums, &mut all_results)
    }

    pub fn trim_nums(nums: &Vec<i32>) -> Vec<i32> {
        let mut nums_set: HashMap<i32, i32> = std::collections::HashMap::new();
        let mut new_nums = vec![];
        for num in nums {
            nums_set.insert(num.clone(), nums_set.get(num).unwrap_or(&0) + 1);
            let size = nums_set.get(num).unwrap_or(&0).clone();
            if size <= 4 {
                new_nums.push(num.clone());
            }
        }
        new_nums
    }


    pub fn map_to_num(nums: &Vec<i32>, pos_results: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut num_results = std::collections::HashSet::new();
        for pos_result in pos_results {
            let mut num_result = vec![];
            for pos in pos_result {
                num_result.push(nums[pos.clone() as usize]);
            }
            num_result.sort();
            num_results.insert(num_result);
        }
        num_results.into_iter().collect()
    }

    pub fn processor(
        nums: &Vec<i32>,
        start_pos: usize,
        level: usize,
        target_num: i32,
        result: HashSet<i32>,
        all_results: &mut Vec<Vec<i32>>,
    ) {
        println!("processor");
        if level <= 1 {
            return;
        } else if level == 2 {
            let two_sum_results = Self::two_sum(nums, start_pos,target_num);
            for two_sum_result in two_sum_results {
                let mut tmp_result = result.clone();
                tmp_result.insert(two_sum_result.0 as i32);
                tmp_result.insert(two_sum_result.1 as i32);
                if tmp_result.len() == 4 {
                    all_results.push(tmp_result.into_iter().collect())
                }
            }
        } else {
            for i in 0..nums.len() {
                if result.contains(&(i as i32)) {
                    continue;
                }
                let num = nums[i];
                let mut temp_result = result.clone();
                temp_result.insert(i as i32);
                let target_num= target_num as i64;
                let num = num as i64;
                let next_target_num = target_num - num;
                if next_target_num > i32::MAX as i64 || next_target_num < i32::MIN as i64 {
                    continue;
                }
                Self::processor(nums, i + 1,level - 1, (target_num - num) as i32, temp_result.clone(), all_results)
            }
        }
    }

    /// return <pos1, pos2>
    pub fn two_sum(nums: &Vec<i32>, start_pos: usize, target: i32) -> Vec<(usize, usize)> {
        let mut result = vec![];
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in start_pos..nums.len() {
            let i_num = nums[i];
            let another_num = target - i_num;
            if let Some(another) = map.get(&another_num) {
                for an in another {
                    result.push((an.clone(), i));
                }
            }

            match map.get(&i_num) {
                Some(i_s) => {
                    let mut mut_i_s = i_s.clone();
                    mut_i_s.push(i);
                    map.insert(i_num, mut_i_s);
                },
                None => {
                    map.insert(i_num, vec![i]);
                },
            }
        }
        result
    }
}
// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::four_sum(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
