/*
Given a list of integers, write a function that returns the largest sum of non-adjacent numbers. Numbers can be 0 or negative.
For example, [2, 4, 6, 2, 5] should return 13, since we pick 2, 6, and 5. [5, 1, 1, 5] should return 10, since we pick 5 and 5.

Solution:
    First, you never want to add on a negative number, so set all negatives to 0 (so adding them is
        effectively the same as skipping them)

    Second, observe that given an extra value, you will either want to add it to the highest value
        from two before, or three values before. You can't add it to the value one before (because
        the problem said so) and adding to four before is always worse than adding to two before
        since two before has already added onto four before.

    Therefore, at any time, there are four possible best options:
        two values for the current value (2/3 before), and two values from skipping the current value
        and using one of the two best options from the previous number.


*/

pub fn max_sum(input: &Vec<i32>) -> i32 {
    // remove negative numbers
    let input:Vec<i32> = input.iter().map(|&v| v.max(0)).collect();

    let mut bests = (0, 0);  // (2 before, 3 before)
    let mut bests_other = (0, 0); // (2 before, 3 before)
    for v in input {

        // increase the current value possibilities
        bests.0 += v;
        bests.1 += v;

        // switch the best pointers and update them ready for next round
        std::mem::swap(&mut bests, &mut bests_other);
        bests.0 = bests.0.max(bests.1);
        bests_other.1 = bests.0;
    }

    // get the best of the values
    bests.0.max(bests.1).max(bests_other.0.max(bests_other.1))

}

#[cfg(test)]
mod test {
    use max_sum;

    #[test]
    fn test_1() {
        let input = vec![2, 4, 6, 2, 5];
        assert_eq!(max_sum(&input), 13);
    }

    #[test]
    fn test_2() {
        let input = vec![5, 1, 1, 5];
        assert_eq!(max_sum(&input), 10);
    }

    #[test]
    fn test_3() {
        let input = vec![500, 1, 1, 100, 5];
        assert_eq!(max_sum(&input), 600);
    }

    #[test]
    fn test_4() {
        let input = vec![1, 500, 1, 100, 5];
        assert_eq!(max_sum(&input), 600);
    }

    #[test]
    fn test_5() {
        let input = vec![-1, -1, 3, -3, 5];
        assert_eq!(max_sum(&input), 8);
    }

    #[test]
    fn test_6() {
        let input = vec![-1, -1, -3, -3, -5];
        assert_eq!(max_sum(&input), 0);
        // maybe this should be -1?
        // I figure we can sum none of them to get 0
    }

}