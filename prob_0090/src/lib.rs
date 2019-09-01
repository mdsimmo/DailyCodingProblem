/*

Given an integer n and a list of integers l, write a function that randomly generates a number
from 0 to n-1 that isn't in l (uniform).

Time to complete: 47 min
*/

extern crate rand;

use std::collections::BTreeSet;
use rand::prelude::*;

pub fn rand(n: usize, list: &[usize]) -> usize {
    // sort the list and remove duplicates
    let sorted: BTreeSet<_> = list.iter().collect();

    // count the number of spaces between 0 and n by finding the index of the first value >= n
    let spaces = match sorted.iter().enumerate().filter(|(_, &v)| v >= &n).next() {
        Some((i, _)) => n - i,
        None => n,
    };

    // Choose a number between [0, spaces) (will crash if spaces == 0)
    let mut rnd = thread_rng();
    let generated = rnd.gen_range(0, spaces);

    // add the spaces back onto 'generated'
    sorted.iter().fold(generated,
               |generated, &x|
                   if x <= &generated {
                       generated + 1
                   } else {
                       generated
                   }
    )
}

#[cfg(test)]
mod tests {
    use rand;
    use std::collections::BTreeSet;

    #[test]
    fn test_thing() {
        let input = &[5, 7, 2, 9, 5, 4, 0, 2];
        let max = 8;

        let results: BTreeSet<usize> = (0..100).into_iter().to_owned().map(|_| rand(max, input)).collect();
        let results2: Vec<usize> = results.into_iter().collect();

        assert_eq!(&[1, 3, 6], &results2[..]);
    }

    #[test]
    fn test_max_equal_to_num() {
        let input = &[1, 3, 4, 5];
        let max = 4;

        let results: BTreeSet<usize> = (0..100).into_iter().to_owned().map(|_| rand(max, input)).collect();
        let results2: Vec<usize> = results.into_iter().collect();

        assert_eq!(&[0, 2], &results2[..]);
    }
}