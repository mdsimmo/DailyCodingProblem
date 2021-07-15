/*

Find an efficient algorithm to find the smallest distance (measured in number of words) between
any two given words in a string.

For example, given words "hello", and "world" and a text content of "dog cat hello cat dog dog
hello cat world", return 1 because there's only one word "cat" in between the two words.

Time to complete: 1h 3min
*/
extern crate regex;

use regex::Regex;

pub fn distance(word1: &str, word2: &str, sentence: &str) -> Option<usize> {
    let mut m1 = sentence.match_indices(&word1pat[..]);
    let mut m2 = sentence.match_indices(&word2pat[..]);

    let mut op1 = m1.next();
    let mut op2 = m2.next();

    let mut min_dist = None;

    while op1.is_some() && op2.is_some() {
        let (p1, _) = op1.unwrap();
        let (p2, _) = op2.unwrap();

        let regex = Regex::new("\\s+").unwrap();
        let dist = regex.find_iter(&sentence[if p1 < p2 { p1..p2 } else { p2..p1 }]).count() - 1;
        min_dist = Some(min_dist.map_or_else(|| dist, |x| if dist < x { dist } else { x }));

        if p1 < p2 {
            op1 = m1.next()
        } else {
            op2 = m2.next();
        }
    }

    return min_dist;
}

#[cfg(test)]
mod tests {
    use distance;

    #[test]
    fn test_given() {
        assert_eq!(Some(1), distance("hello", "world", "dog cat hello cat dog dog hello cat world"))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(0), distance("hello", "world", "dog cat hello cat dog world hello cat world"))
    }

    #[test]
    fn test2() {
        assert_eq!(Some(4), distance("hello", "world", " dog world dogs Cats Pats cat hello"))
    }

    #[test]
    fn test_doublespace() {
        assert_eq!(Some(0), distance("a", "b", "a  b"))
    }

    #[test]
    fn test_none() {
        assert_eq!(None, distance("hello", "world", "dog cat cat dog world cat world"))
    }
}