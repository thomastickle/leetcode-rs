/// LeetCode 643: Maximum Average Subarray I

struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let n = nums.len();
        
        if n < k {
            return 0.0;
        }

        let mut curr_sum: i32 = nums[0..k].iter().sum();
        let mut max_sum = curr_sum;
        
        
        for i in k..n {
            curr_sum += nums[i] - nums[i - k];
            max_sum = max_sum.max(curr_sum);
        }
        
        max_sum as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::lc_643::Solution;
    use float_eq::assert_float_eq;

    #[test]
    fn test_find_max_average() {
        const MAX_ERROR: f64 = 1e-5;

        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        let results = Solution::find_max_average(nums, k);
        assert_float_eq!(results, 12.750000, abs <= MAX_ERROR);

        let nums = vec![5];
        let k = 1;
        let results = Solution::find_max_average(nums, k);
        assert_float_eq!(results, 5.000000, abs <= MAX_ERROR)
    }
}
