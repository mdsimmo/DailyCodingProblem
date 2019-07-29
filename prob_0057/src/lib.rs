/*
Given a string s and an integer k, break up the string into multiple lines such that each line has a length of k or less. You must break it up so that words don't break across lines. Each line has to have the maximum possible amount of words. If there's no way to break the text up, then return null.

You can assume that there are no spaces at the ends of the string and that there is exactly one space between each word.

For example, given the string "the quick brown fox jumps over the lazy dog" and k = 10

Time to complete: 1h
*/

pub fn splitify(input: &str, max_length: usize) -> Vec<&str> {

    let mut results = vec![];

    let mut last_full = 0..0;
    let mut last_word = 0..0;

    for (i, c) in input.chars().enumerate() {
        if c == ' ' {
            let size = last_word.end - last_full.start;
            if size > max_length {
                results.push(&input[last_full]);
                last_full = last_word;
            } else {
                last_full.end = last_word.end;
            }
            last_word = i+1..i+1;
        } else {
            last_word.end = i+1;
        }
    }

    let size = last_word.end - last_full.start;
    if size > 0 {
        if size > max_length {
            results.push(&input[last_full]);
            results.push(&input[last_word]);
        } else {
            results.push(&input[last_full.start..last_word.end])
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use splitify;

    #[test]
    fn test_given() {
        let input = "the quick brown fox jumps over the lazy dog";
        let k = 10;

        let expected = vec!["the quick", "brown fox", "jumps over", "the lazy", "dog"];
        assert_eq!(expected, splitify(input, k));
    }

    #[test]
    fn test_words_too_big() {
        let input = "hello reallylargebrown fox";
        let k = 10;

        let expected = vec!["hello", "reallylargebrown", "fox"];
        assert_eq!(expected, splitify(input, k));
    }

    #[test]
    fn test_words_fits_perfeclty() {
        let input = "12345 12345";
        let k = 5;

        let expected = vec!["12345", "12345"];
        assert_eq!(expected, splitify(input, k));
    }

    #[test]
    fn test_empty() {
        let input = "";
        let k = 5;

        let expected: Vec<&str> = vec![];
        assert_eq!(expected, splitify(input, k));
    }
}