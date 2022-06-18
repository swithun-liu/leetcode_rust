pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {

    fn get_let(digits: u8) -> Vec<char> {
        match digits {
            b'2' => vec!['a', 'b', 'c'],
            b'3' => vec!['d', 'e', 'f'],
            b'4' => vec!['g', 'h', 'i'],
            b'5' => vec!['j', 'k', 'l'],
            b'6' => vec!['m', 'n', 'o'],
            b'7' => vec!['p', 'q', 'r', 's'],
            b'8' => vec!['t', 'u', 'v'],
            b'9' => vec!['w', 'x', 'y', 'z'],
            _ => vec![]
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        /// step1: convert number string to number vector
        // let vec_digits_u32: Vec<u32> = digits.chars().map(|c:char| c.to_digit(10).unwrap()).collect();
        let vec_digits: Vec<u8> = digits.into_bytes();

        /// step2: check if there is no number
        if vec_digits.is_empty() {
            return vec![];
        }

        // step3: do the calculation
        let mut current_str = String::new();
        Self::deal_with_one_level(&vec_digits, &mut result, &mut current_str);

        result
    }

    fn deal_with_one_level(vec_digits: &[u8], vec_result: &mut Vec<String>, current_str: &mut String) {
        if let Some((first_digit, rest_digits)) = vec_digits.split_first() {
            let vec_corresponding_chars = Self::get_let(*first_digit);
            for ch in &vec_corresponding_chars {
                current_str.push(*ch);
                Self::deal_with_one_level(rest_digits, vec_result, current_str);
                current_str.pop();
            }
        } else {
            vec_result.push(current_str.clone());
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn letter_combinations(digits: String) -> Vec<String> {
        Self::letter_combinations(digits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
