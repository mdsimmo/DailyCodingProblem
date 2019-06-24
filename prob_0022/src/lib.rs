/**

Given a dictionary of words and a string made up of those words (no spaces), return the original
sentence in a list. If there is more than one possible reconstruction, return any of them. If there
 is no possible reconstruction, then return null.

For example, given the set of words 'quick', 'brown', 'the', 'fox', and the string "thequickbrownfox",
 you should return ['the', 'quick', 'brown', 'fox'].

Given the set of words 'bed', 'bath', 'bedbath', 'and', 'beyond', and the string "bedbathandbeyond",
 return either ['bed', 'bath', 'and', 'beyond] or ['bedbath', 'and', 'beyond'].


Time to complete: 30min

*/

pub fn words<'a>(dict: &'a [&str], sentence: &str) -> Vec<Vec<&'a str>> {
    let mut results = vec![];
    for &word in dict {
        if sentence.ends_with(word) {
            let (head, _) = sentence.split_at(sentence.len()-word.len());
            if head.is_empty() {
                results.push(vec![word]);
            } else {
                let mut possibilities: Vec<Vec<&str>> = words(dict, head);
                for data in possibilities.iter_mut() {
                    data.push(word);
                }
                results.append(&mut possibilities);
            }
        }
    }
    return results
}


#[cfg(test)]
mod test {
    use words;

    #[test]
    fn test_given_1() {
        let dict = ["quick", "brown", "the", "fox"];
        let sentence = "thequickbrownfox";

        assert_eq!(words(&dict, &sentence), vec![
            vec!["the", "quick", "brown", "fox"]
        ]);
    }

    #[test]
    fn test_given_2() {
        let dict = ["bed", "bath", "bedbath", "and", "beyond"];
        let sentence = "bedbathandbeyond";

        assert_eq!(words(&dict, &sentence), vec![
            vec!["bed", "bath", "and", "beyond"],
            vec!["bedbath", "and", "beyond"]
        ]);
    }

    #[test]
    fn test_deadend() {
        let dict = ["this", "will", "stop"];
        let sentence = "thiswillstopinthemiddle";

        let result: Vec<Vec<&str>> = vec![];
        assert_eq!(words(&dict, &sentence), result);
    }

    #[test]
    fn test_some_deadends() {
        let dict = ["this", "will", "stop", "in", "the", "middle", "stopin", "stopinth"];
        let sentence = "thiswillstopinthemiddle";

        assert_eq!(words(&dict, &sentence), vec![
                vec!["this", "will", "stop", "in", "the", "middle"],
                vec!["this", "will", "stopin", "the", "middle"],
        ]);
    }
}