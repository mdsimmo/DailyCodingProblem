/*

Given a linked list and a positive integer k, rotate the list to the right by k places.

For example, given the linked list 7 -> 7 -> 3 -> 5 and k = 2, it should become 3 -> 5 -> 7 -> 7.

Given the linked list 1 -> 2 -> 3 -> 4 -> 5 and k = 3, it should become 3 -> 4 -> 5 -> 1 -> 2.

Time to complete: 9 min
*/

pub fn rotate<T>(input: &mut Vec<T>, k: usize) {
    for _ in 0..k {
        if let Some(v) = input.pop() {
            input.insert(0, v)
        }
    }
}

#[cfg(test)]
mod tests {
    use rotate;

    #[test]
    fn test_given_1() {
        let mut input = vec![7, 7, 3, 5];
        rotate(&mut input, 2);
        assert_eq!(vec![3, 5, 7, 7], input);
    }

    #[test]
    fn test_given_2() {
        let mut input = vec![1, 2, 3, 4, 5];
        rotate(&mut input, 3);
        assert_eq!(vec![3, 4, 5, 1, 2], input);
    }
}