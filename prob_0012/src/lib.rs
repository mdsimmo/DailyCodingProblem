/*
There exists a staircase with N steps, and you can climb up either 1 or 2 steps at a time. Given N, write a function that returns the number of unique ways you can climb the staircase. The order of the steps matters.

For example, if N is 4, then there are 5 unique ways:

    1, 1, 1, 1
    2, 1, 1
    1, 2, 1
    1, 1, 2
    2, 2

What if, instead of being able to climb 1 or 2 steps at a time, you could climb any number from a set of positive integers X? For example, if X = {1, 3, 5}, you could climb 1, 3, or 5 steps at a time.

Time to complete: 1h 25min
*/

pub fn possibilities(steps_remaining: u32, step_choices: &[u32]) -> u32 {
    if steps_remaining == 0 {
        return 1;
    }

    step_choices.iter()
        .filter(|&i| i <= &steps_remaining)
        .map(|&step| possibilities(steps_remaining-step, step_choices))
        .sum()
}


#[cfg(test)]
pub mod test {
    use possibilities;

    #[test]
    fn test_example() {
        assert_eq!(possibilities(4, &[1, 2]), 5);
    }

    #[test]
    fn test_large_steps() {
        assert_eq!(possibilities(4, &[1, 2, 3, 4]), 8);
    }

    #[test]
    fn test_one_step() {
        assert_eq!(possibilities(1, &[1, 2]), 1);
    }

    #[test]
    fn test_two_step() {
        assert_eq!(possibilities(2, &[1, 2]), 2);
    }

    #[test]
    fn test_not_even_steps() {
        assert_eq!(possibilities(3, &[1, 3]), 2);
    }

    #[test]
    fn impossible() {
        assert_eq!(possibilities(3, &[2]), 0);
    }

}