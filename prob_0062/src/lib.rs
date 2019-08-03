/*
There is an N by M matrix of zeroes. Given N and M, write a function to count the number of ways of starting at the top-left corner and getting to the bottom-right corner. You can only move right or down.

For example, given a 2 by 2 matrix, you should return 2, since there are two ways to get to the bottom-right:

    Right, then down
    Down, then right

Given a 5 by 5 matrix, there are 70 ways to get to the bottom-right.

Time to complete: 6 min
*/

pub fn possibilities(width: u64, height: u64) -> u64 {

    if width == 1 || height == 1 {
        1
    } else {
        // there's probably a formula for this, but, I cant be stuffed finding it
        possibilities(width - 1, height) + possibilities(width, height - 1)
    }
}


#[cfg(test)]
mod tests {
    use possibilities;

    #[test]
    fn test_one_by_one() {
        assert_eq!(1, possibilities(1, 5));
        assert_eq!(1, possibilities(5, 1));
    }

    #[test]
    fn test_two_by_two() {
        assert_eq!(2, possibilities(2, 2));
    }

    #[test]
    fn test_five_by_five() {
        assert_eq!(70, possibilities(5, 5));
    }

    #[test]
    fn test_rectangle() {
        assert_eq!(3, possibilities(2, 3));
    }
}