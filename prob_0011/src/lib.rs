
/*
Implement an autocomplete system. That is, given a query string s and a set of all possible query strings, return all strings in the set that have s as a prefix.

For example, given the query string de and the set of strings [dog, deer, deal], return [deer, deal].

Hint: Try preprocessing the dictionary into a more efficient data structure to speed up queries.
*/


#[derive(Debug, PartialEq)]
pub struct Set<'a> {
    set: Vec<&'a str>
}

impl <'a> Set<'a> {

    pub fn new(set: &Vec<&'a str>) -> Self {
        let mut set_ordered = set.clone();
        set_ordered.sort();
        Self { set: set_ordered }
    }

    pub fn query(&self, prefix: &str) -> &[&str] {
        // The set of words is ordered. Therefore, find the first and last matching
        // indices (by binary search), and then return everything in between.
        // Thus, this is a O(ln(n)) operation.

        // Indexing will crash if prefix is empty. So handle that separately
        if prefix.is_empty() {
            return self.set.as_slice()
        }

        let start_mark = prefix;

        // replace the last character in start_mark with one letter higher
        let mut end_mark = start_mark.to_owned();
        end_mark.remove(end_mark.len()-1);
        end_mark.push((start_mark.chars().last().unwrap() as u8 + 1) as char);
        let end_mark = &end_mark[..];

        let start_index = match self.set.binary_search_by(|&s| s.cmp(prefix)) {
            Ok(n) => n,
            Err(n) => n,
        };
        let end_index = match self.set.binary_search_by(|&s| s.cmp(end_mark)) {
            Ok(n) => n,
            Err(n) => n
        };

        self.set.split_at(start_index).1.split_at(end_index-start_index).0
    }
}

#[cfg(test)]
mod test {
    use Set;

    #[test]
    fn test_retrieve() {
        let set = Set::new(&vec!["dog", "deer", "deal"]);

        assert_eq!(set.query("de"), &["deal", "deer"]);
        assert_eq!(set.query("do"), &["dog"]);
    }

    #[test]
    fn test_retrieve_with_empty_prefix() {
        let set = Set::new(&vec!["dog", "deer", "deal"]);

        assert_eq!(set.query(""), &["deal", "deer", "dog"]);
    }

    #[test]
    fn test_retrieve_with_empty_element() {
        let set = Set::new(&vec!["", "aabc", "abc", "a"]);

        assert_eq!(set.query("ab"), &["abc"]);
    }

    #[test]
    fn test_nothing_to_return() {
        let set = Set::new(&vec!["one", "two", "three", "four"]);

        assert_eq!(set.query("five"), &[] as &[&str]);
    }

    #[test]
    fn test_empty_input() {
        let set = Set::new(&vec![]);

        assert_eq!(set.query("bob"), &[] as &[&str]);
    }

    #[test]
    fn test_prefix_matches_word() {
        let set = Set::new(&vec!["a", "b", "bob", "bab", "c"]);

        assert_eq!(set.query("b"), &["b", "bab", "bob"]);
        assert_eq!(set.query("bab"), &["bab"]);
        assert_eq!(set.query("bob"), &["bob"]);
    }
}