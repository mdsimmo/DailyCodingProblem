/*
A number is considered perfect if its digits sum up to exactly 10.

Given a positive integer n, return the n-th perfect number.

For example, given 1, you should return 19. Given 2, you should return 28.

Time to complete: 7 min
*/

pub fn nth_perfect(mut index: usize) -> u64 {
    let mut count = 18;
    loop {
        if sum_digits(count) == 10 {
            index -= 1;
            if index == 0 {
                return count
            }
        }
        count += 1;
    }
}

fn sum_digits(mut num: u64) -> u64 {
    let mut sum = 0;
    loop {
        let div = num / 10;
        sum += num % 10;
        if div == 0 {
            return sum;
        } else {
            num = div;
        }
    }
}


#[cfg(test)]
mod tests {
    use ::{nth_perfect, sum_digits};

    #[test]
    fn test_sum_digits() {
        assert_eq!(5+4+3+2, sum_digits(5432));
    }

    #[test]
    fn test_one() {
        assert_eq!(19, nth_perfect(1));
    }

    #[test]
    fn test_two() {
        assert_eq!(28, nth_perfect(2));
    }

    #[test]
    fn test_nine() {
        assert_eq!(91, nth_perfect(9));
    }

    #[test]
    fn test_10() {
        assert_eq!(109, nth_perfect(10));
    }

}