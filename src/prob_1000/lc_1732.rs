use crate::lc_lib::solution::Solution;

/// LeetCode 1732: Find the Highest Altitude

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (mut current_altitude, mut max_altitude) = (0, 0);
        
        for i in gain {
            current_altitude = current_altitude + i;
            max_altitude = max_altitude.max(current_altitude);
        }
        max_altitude
    }
}

#[test]
fn test_largest_altitude() {
    let gain = vec![-5,1,5,0,-7];
    let result = Solution::largest_altitude(gain);
    assert_eq!(result, 1);
    
    let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    let result = Solution::largest_altitude(gain);
    assert_eq!(result, 0);
}
