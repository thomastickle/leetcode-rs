struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];

        let mut left = 1;
        let mut right = 1;

        for i in 0..n {
            result[i] *= left;
            left *= nums[i];

            result[n - i - 1] *= right;
            right *= nums[n - i - 1];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::lc_238::Solution;

    #[test]
    fn test_product_except_self() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);

        let nums = vec![-1, 1, 0, -3, 3];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }
}
