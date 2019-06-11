pub fn possibilities(input: &[u8]) -> u64 {
    if let Some((first, tail)) = input.split_first() {
        match first {
            0 => 0,
            d@1..=9 => {
                // count possibilities as a single digit
                let as_d = possibilities(tail).max(1);

                // count possibilities as a double digit number
                let as_dx= if let Some((n, t)) = tail.split_first() {
                    if d*10 + n > 26 {
                        0
                    } else {
                        possibilities(t).max(1)
                    }
                } else {
                    0
                };

                // return the sum of the two possibilities
                as_d+as_dx
            },
            _ => panic!("Input numbers must be in [0-9]"),
        }
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use ::*;

    #[test]
    fn test_one_one() {
        let input = vec![1];
        assert_eq!(1, possibilities(&input))
    }

    #[test]
    fn test_two_ones() {
        let input = vec![1, 1];
        assert_eq!(2, possibilities(&input))
    }

    #[test]
    fn test_zero_in_center() {
        let input = vec![2, 0, 2];
        assert_eq!(2, possibilities(&input))
    }

    #[test]
    fn test_all_ones() {
        let input = vec![1, 1, 1];
        assert_eq!(3, possibilities(&input))
    }

    #[test]
    fn test_something() {
        let input = vec![3, 1, 2, 1];
        assert_eq!(3, possibilities(&input))
    }

    #[test]
    fn test_twenty_less_than_eq_6() {
        let input = vec![2, 6];
        assert_eq!(2, possibilities(&input))
    }

    #[test]
    fn test_twenty_more_than_6() {
        let input = vec![2, 7];
        assert_eq!(1, possibilities(&input))
    }

    #[test]
    fn test_some_stuff() {
        let input = vec![4, 2, 1, 2, 7, 0];
        assert_eq!(3, possibilities(&input))
    }

    #[test]
    fn test_only_one_start_options() {
        let input = vec![3, 1, 1];
        assert_eq!(2, possibilities(&input))
    }

    #[test]
    fn test_zeros_has_none() {
        let input = vec![0, 1, 1];
        assert_eq!(0, possibilities(&input))
    }

    #[test]
    fn test_single_value() {
        let input = vec![4];
        assert_eq!(1, possibilities(&input))
    }

    #[test]
    fn performance_doesnt_degrade_badly() {
        let input = vec![1, 1, 1, 8, 5, 3, 5, 8, 2, 3, 1, 7, 5, 3, 2, 9, 7, 1, 5, 4, 1, 2];
        assert_eq!(80, possibilities(&input))
    }
}