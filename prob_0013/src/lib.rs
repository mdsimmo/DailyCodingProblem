use std::collections::HashSet;

/**
Given an integer k and a string s, find the length of the longest substring that contains at most k distinct characters.

For example, given s = "abcba" and k = 2, the longest substring with k distinct characters is "bcb".


Solution notes:
    This solution probably isn't the best; it requires a fair bit of iteration over the
    input string (note that every s.chars().nth() or s[a..b] call requires iteration from
    the start of the string since one char might not be one byte).

    It also isn't particularly clean (lots of indexes and fidgety stuff)

*/

pub fn longest(s: &str, k: usize) -> &str {

    let mut best_start = 0;
    let mut best_end = 0;

    let mut start_index = 0;
    let mut chars = HashSet::with_capacity(k);

    for (i, c) in s.chars().enumerate() {
        chars.insert(c);

        if chars.len() > k {
            // shorten the string down so it no longer includes the first letter
            let first = s.chars().nth(start_index).unwrap();
            chars.remove(&first);
            for (i2, c2) in s[start_index..=i].chars().rev().enumerate() {
                if c2 == first {
                    start_index = i-i2+1;
                    break;
                }
            }
        }

        // check if we're the best
        if (i+1-start_index) > (best_end-best_start) {
            best_end = i+1;
            best_start = start_index;
        }
    }

    return &s[best_start..best_end];
}


#[cfg(test)]
pub mod test {
    use longest;

    #[test]
    fn test_given() {
        assert_eq!(longest("abcba", 2), "bcb");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(longest("", 2), "");
    }

    #[test]
    fn test_one_char() {
        assert_eq!(longest("abcaccfg", 1), "cc");
    }

    #[test]
    fn test_zero_chars() {
        assert_eq!(longest("aa", 0), "");
    }

}