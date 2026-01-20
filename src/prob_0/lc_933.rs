use std::collections::VecDeque;

/// LeetCode 933: Number of Recent Calls

struct RecentCounter {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    const MAX_AGE: i32 = 3000;

    fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while let Some(&front) = self.queue.front() {
            if t - front > Self::MAX_AGE {
                self.queue.pop_front();
            } else {
                break;
            }
        }

        self.queue.push_back(t);
        self.queue.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod tests {

    #[test]
    fn test_recent_counter() {
        use super::RecentCounter;

        let mut obj = RecentCounter::new();
        let ret_1: i32 = obj.ping(1);
        let ret_2: i32 = obj.ping(100);
        let ret_3: i32 = obj.ping(3001);
        let ret_4: i32 = obj.ping(3002);

        assert_eq!(ret_1, 1);
        assert_eq!(ret_2, 2);
        assert_eq!(ret_3, 3);
        assert_eq!(ret_4, 3);
    }
}
