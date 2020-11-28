pub struct Solution;

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let num = (num + 1) as _;
        let mut result = vec![0; num];

        for i in 1..num {
            result[i] = result[i >> 1] + (i & 1) as i32;
        }

        result
    }
}

impl super::Solution for Solution {
    fn count_bits(num: i32) -> Vec<i32> {
        Self::count_bits(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
