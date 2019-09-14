/*

Given a string and a set of characters, return the shortest substring containing all the characters in the set.

For example, given the string "figehaeci" and the set of characters {a, e, i}, you should return "aeci".

If there is no substring containing all the characters in the set, return null.

Time to complete: 1h 5m
*/

use std::collections::{HashSet, HashMap};
use std::ops::Range;

#[macro_export]
macro_rules! set {
    [$($x:expr),*] => {{
        let mut set = HashSet::new();
        $(set.insert($x);)*
        set
    }}
}

pub fn shortest_substring<'a>(string: &'a str, chars: &HashSet<char>) -> Option<&'a str> {
    // build a lookup of char to indices
    let mut lookup: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in string.chars().enumerate() {
        if chars.contains(&c) {
            match lookup.get_mut(&c) {
                Some(vec) => {vec.push(i);},
                None => {lookup.insert(c, vec![i]);},
            };
        }
    }

    // Ensure that all the chars are present
    for c in chars {
        if !lookup.contains_key(c) {
            return None;
        }
    }

    // Find the shortest set of letters
    let indexes = shortest(&lookup, vec![]);
    Some(&string[indexes])
}

fn shortest(chars: &HashMap<char, Vec<usize>>, assigned: Vec<usize>) -> Range<usize> {
    let entry = chars.iter().next();
    match entry {
        Some((c, c_indexes)) => {
            let mut submap = chars.clone();
            submap.remove(c);
            c_indexes.iter()
                .map(|&index| {
                    let mut sub_assigned = assigned.clone();
                    sub_assigned.push(index);
                    let subbest = shortest(&submap, sub_assigned);
                    subbest
                })
                .min_by(|a, b| {
                    (a.end - a.start).cmp(&(b.end-b.start))
                })
                .unwrap()
        },
        None => {
            let min = *assigned.iter().min().unwrap();
            let max = *assigned.iter().max().unwrap() + 1;
            min..max
        }
    }
}


#[cfg(test)]
mod tests {
    use shortest_substring;
    use std::collections::HashSet;

    #[test]
    fn test_given() {
        assert_eq!(Some("aeci"), shortest_substring("figehaeci", &set!['a', 'e', 'i']));
    }

    #[test]
    fn test_another() {
        assert_eq!(Some("adtg"), shortest_substring("aferthgrdadtgsfag", &set!['t', 'a', 'g']));
    }

    #[test]
    fn test_impossible() {
        assert_eq!(None, shortest_substring("aferthgrdadtgsfag", &set!['z', 'a', 'g']));
    }
}