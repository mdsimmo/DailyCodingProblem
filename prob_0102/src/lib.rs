/*
Given a list of integers and a number K, return which contiguous elements of the list sum to K.

For example, if the list is [1, 2, 3, 4, 5] and K is 9, then it should return [2, 3, 4], since 2 + 3 + 4 = 9.

Time to complete: 15 min
*/

use std::cmp::Ordering;

pub fn continuos_add(nums: &[u32], target: u32) -> Option<&[u32]> {
    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;
    loop {
        match sum.cmp(&target) {
            Ordering::Equal => {
                return Some(&nums[start..end]);
            },
            Ordering::Less => {
                if end >= nums.len() {
                    return None;
                }
                sum += nums[end];
                end += 1;
            }
            Ordering::Greater => {
                sum -= nums[start];
                start += 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use continuos_add;

    #[test]
    fn test_given() {
        assert_eq!(Some(&[2, 3, 4][..]), continuos_add(&[1, 2, 3, 4, 5], 9));
    }

    #[test]
    fn test_impossible() {
        assert_eq!(None, continuos_add(&[1, 3, 5, 3, 5], 10));
    }

    #[test]
    fn test_empty_none() {
        assert_eq!(Some(&[][..]), continuos_add(&[], 0));
    }

    #[test]
    fn test_empty_some() {
        assert_eq!(None, continuos_add(&[], 1));
    }
}