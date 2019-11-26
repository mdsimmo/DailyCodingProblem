/*
Given a list of integers, return the largest product that can be made by multiplying any three integers.

For example, if the list is [-10, -10, 5, 2], we should return 500, since that's -10 * -10 * 5.

You can assume the list has at least three integers.

Time to complete: 7 min
*/

pub fn largest_multiple(numbers: &Vec<i32>) -> i32 {
    let mut largest = vec![0, 0, 0];
    let _a = 10i32.abs();
    for &i in numbers {
        match largest.clone().iter().enumerate().min_by_key(|(_, &x)| x) {
            None => {
                largest.push(i);

            },
            Some((index, &value)) => {
                if largest.len() < 3 {
                    largest.push(i)
                } else if value < i {
                     largest[index] = i;
                }
            }
        }
    }
    largest.iter().fold(1, |a, &b| a * b)
}

#[cfg(test)]
mod tests {
    use largest_multiple;

    #[test]
    fn test_given() {
        assert_eq!(500, largest_multiple(&vec![-10, -10, 5, 2]));
    }

}