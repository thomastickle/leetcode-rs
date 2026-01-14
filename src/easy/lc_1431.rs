

struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies = *candies.iter().max().unwrap();
        candies.iter().map(|num| num + extra_candies >= max_candies).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::lc_1431::Solution;

    #[test]
    fn test_kids_with_candies() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, vec![true, true, true, false, true]);

        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, vec![true, false, false, false, false]);

        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, vec![true, false, true]);
    }
}
