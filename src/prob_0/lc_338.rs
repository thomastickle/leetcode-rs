// LeetCode 338: Counting Bits

use crate::lc_lib::solution::Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0];

        for i in 1..n + 1 {
            let index: usize = (i / 2) as usize;
            let val: i32 = result[index];
            result.push(if i % 2 == 0 { val } else { val + 1 });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::lc_lib::solution::Solution;

    #[test]
    fn test_count_bits() {
        let n = 2;
        let result = Solution::count_bits(n);
        assert_eq!(result, vec![0, 1, 1]);

        let n = 5;
        let result = Solution::count_bits(n);
        assert_eq!(result, vec![0, 1, 1, 2, 1, 2]);
    }
}
