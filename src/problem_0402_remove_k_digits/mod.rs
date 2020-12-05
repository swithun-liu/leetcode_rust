pub mod greedy;
pub mod stack;

pub trait Solution {
    fn remove_kdigits(num: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("1432219", 3), "1219"),
            (("10200", 1), "200"),
            (("10", 2), "0"),
            (("112", 1), "11"),
            (("1173", 2), "11"),
        ];

        for ((num, k), expected) in test_cases.iter().copied() {
            assert_eq!(S::remove_kdigits(num.to_string(), k), expected);
        }
    }
}