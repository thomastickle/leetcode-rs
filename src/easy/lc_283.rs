/// LeetCode 283: Move Zeroes

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        nums.sort_by(|x, y| {
            if *x == 0 && *y != 0 {
                std::cmp::Ordering::Greater
            } else if *x != 0 && *y == 0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::easy::lc_283::Solution;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);

        let mut nums = vec![1, 2, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);

        let mut nums = vec![0, 0, 1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0, 0]);
    }
}
