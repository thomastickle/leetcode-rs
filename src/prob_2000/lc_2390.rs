use crate::lc_lib::solution::Solution;

/// LeetCode 2390: Removing Stars From a String

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = String::new();
        for a_char in s.chars() {
            match a_char {
                '*' => {
                    stack.pop();
                }
                _ => {
                    stack.push(a_char);
                }
            }
        }

        stack
    }
}

#[cfg(test)]
mod tests {
    use crate::lc_lib::solution::Solution;

    #[test]
    fn test_remove_stars() {
        let s = String::from("leet**cod*e");
        let result = Solution::remove_stars(s);
        assert_eq!(result, String::from("lecoe"));

        let s = String::from("erase*****");
        let result = Solution::remove_stars(s);
        assert_eq!(result, String::from(""));
    }
}
