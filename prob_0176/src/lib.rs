/*

Determine whether there exists a one-to-one character mapping from one string s1 to another s2.

For example, given s1 = abc and s2 = bcd, return true since we can map a to b, b to c, and c to d.

Given s1 = foo and s2 = bar, return false since the o cannot map to two characters.

Time to complete: 10 min
*/

use std::collections::HashMap;

pub fn can_map(a: &str, b: &str) -> bool {
    let mut lookup = HashMap::new();

    let mut iter_a = a.chars().into_iter();
    let mut iter_b = b.chars().into_iter();

    while let Some(c_a) = iter_a.next() {
        if let Some(c_b) = iter_b.next() {
            if let Some(c_b_pre) = lookup.insert(c_a, c_b) {
                if c_b_pre != c_b {
                    return false;
                }
            }
        } else {
            return true;
        }
    }
    return true;

}

#[cfg(test)]
mod tests {
    use can_map;

    #[test]
    fn test_given_1() {
        assert!(can_map("abc", "bcd"))
    }

    #[test]
    fn test_given_2() {
        assert!(!can_map("foo", "bar"))
    }

    #[test]
    fn test_multiple_same_characters() {
        assert!(can_map("foop", "baar"))
    }

    #[test]
    fn test_different_length() {
        assert!(can_map("foopy", "baar"));
        assert!(can_map("foop", "baary"));
    }

}