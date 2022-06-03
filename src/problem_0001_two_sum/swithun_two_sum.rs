pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        println!("target: {}", target);
        for num in &nums {
            print!(" {}", *num);
        }
        println!();
        let (last, nums) = nums.split_last().unwrap();
        for num in nums {
            print!(" {}", *num);
        }
        println!();
        println!("last : {}", last);
        let mut indices = HashMap::new();

        for (i, &num) in (0..).zip(nums) {
            match indices.get(&(target - num)) {
                None => indices.insert(num, i),
                Some(&index) => return vec![index, i],
            };

            for (key, value) in &indices {
                print!("<{}, {}> ", key, value);
            }
            println!();

        }

        println!("2---{}", indices[&(target - last)]);

        vec![indices[&(target - last)], nums.len() as _]


    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
