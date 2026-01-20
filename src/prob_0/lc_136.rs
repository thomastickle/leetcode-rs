use crate::lc_lib::solution::Solution;

/// Leet Code 136: Single Number


impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, &x| acc ^ x)
    }
}


#[cfg(test)]
mod tests {
    use crate::lc_lib::solution::Solution;

    #[test]
    fn test_single_number() {
        let nums = vec![2, 2, 1];
        let result = Solution::single_number(nums);
        assert_eq!(result, 1);

        let nums = vec![4, 1, 2, 1, 2];
        let result = Solution::single_number(nums);
        assert_eq!(result, 4);

        let nums = vec![1];
        let result = Solution::single_number(nums);
        assert_eq!(result, 1);
    }
}
