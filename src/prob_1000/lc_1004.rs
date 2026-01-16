use crate::lc_lib::solution::Solution;

// LeetCode 1004: Max Consecutive Ones III

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut window_start, mut longest, mut zero_count) = (0, 0, 0);

        for (window_index, &num) in nums.iter().enumerate() {
            if num == 0 {
                zero_count += 1;
            }

            while zero_count == k + 1 {
                if nums[window_start] == 0 {
                    zero_count -= 1;
                }
                window_start += 1;
            }
            longest = longest.max(window_index - window_start + 1)
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::lc_lib::solution::Solution;

    #[test]
    pub fn test_longest_ones() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let max_swaps = 2;
        let result = Solution::longest_ones(nums, max_swaps);
        assert_eq!(result, 6);

        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let max_swaps = 3;
        let result = Solution::longest_ones(nums, max_swaps);
        assert_eq!(result, 10);
    }
}
