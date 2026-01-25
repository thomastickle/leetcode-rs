use crate::lc_lib::solution::Solution;

/// LeetCode 206: Reverse Linked List
/// 


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  #[allow(dead_code)]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl Solution {
    
    
    
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse_list_helper(current: Option<Box<ListNode>>, previous: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match current {
                None => previous,
                Some(mut node) => {
                    let next = node.next.take();
                    node.next = previous;
                    reverse_list_helper(next, Some(node))
                }
            }
        }
        
        reverse_list_helper(head, None)        
    }
}



#[cfg(test)]
mod tests {
    use crate::lc_lib::solution::Solution;
    use crate::prob_0::lc_206::ListNode;

    fn make_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        if vec.is_empty() {
            return None;
        }
        
        vec.iter().rev().fold(None, |acc, &val| {
            Some(Box::new(ListNode {
                val,
                next: acc,
            }))
        })
    }
    
    #[test]
    fn test_make_list() {
        let list = [1,2,3];
        let head = make_list(Vec::from(list));
        assert_eq!(head.as_ref().unwrap().val, 1);
        assert_eq!(head.as_ref().unwrap().next.as_ref().unwrap().val, 2);
        assert_eq!(head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().val, 3);
        assert_eq!(head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().next, None);
    }
    
    #[test]
    fn test_reverse_list() {
        let list = [1,2,3,4,5];
        let head = make_list(Vec::from(list));
        let option = Solution::reverse_list(head);
        assert_eq!(option.unwrap().val, 5);
        
        
    }
}
