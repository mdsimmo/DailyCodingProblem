/*
A knight's tour is a sequence of moves by a knight on a chessboard such that all squares are visited once.

Given N, write a function to return the number of knight's tours on an N by N chessboard.

Time to complete: 1h 30m

This is by no means the fastest solution, but it does work
*/

extern crate ndarray;
extern crate rayon;

use std::ops::Add;
use ndarray::Array2;
use rayon::prelude::*;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        return Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

pub fn possibilities(size: usize) -> usize {
    let visited = Array2::<bool>::from_elem((size, size), false);

    let is_even = size % 2 == 0;

    let half_size = size / 2;
    let mid_size = if is_even {
        half_size
    } else {
        half_size + 1
    };

    (0..mid_size).into_par_iter().map(|x| (0..mid_size).into_par_iter().map(|y| {

        // only do cells in one eighth of the puzzle
        if x >= mid_size || y >= mid_size || y > x {
            return 0;
        }

        let pos = options(&visited, &Point { x: x as i32, y: y as i32 }, size * size);

        // Some cells cross multiple eights, so need to weight the cells calculations
        let weighting = if x == half_size && y == half_size {
            1 // center square
        } else if x == half_size || y == half_size {
            4 // center line square
        } else if x == y {
            4 // diagonal square
        } else {
            8
        };

        pos * weighting
    }).sum::<usize>()).sum::<usize>()
}

fn options(visited: &Array2<bool>, loc: &Point, remaining: usize) -> usize {

    // Check if we have finished
    if remaining == 1 {
        return 1;
    }

    // update the visited map
    let mut next_visited = visited.clone();
    next_visited[(loc.x as usize, loc.y as usize)] = true;

    let choices: Vec<Point> = vec![
        Point { x:  1, y:  2},
        Point { x:  1, y: -2},
        Point { x: -1, y:  2},
        Point { x: -1, y: -2},
        Point { x:  2, y:  1},
        Point { x:  2, y: -1},
        Point { x: -2, y:  1},
        Point { x: -2, y: -1},
    ];

    let mut sum = 0;

    for p in &choices {
        let next = loc + p;
        if next.x < 0 || next.y < 0 {
            continue;
        }
        let pos = visited.get((next.x as usize, next.y as usize))
            .to_owned()
            .map_or(0, |been_there|
                match been_there {
                    false => {
                        options(&next_visited, &next, remaining - 1)
                    },
                    true => {
                        0
                    }
                }
            );

        sum += pos;
    }
    return sum;
}


#[cfg(test)]
mod tests {
    use possibilities;

    #[test]
    fn test_one_by_one() {
        assert_eq!(possibilities(1), 1);
    }

    #[test]
    fn test_two_by_two() {
        assert_eq!(possibilities(2), 0);
    }

    #[test]
    fn test_five_by_five() {
        assert_eq!(possibilities(5), 1_728);
    }

    #[test]
    fn test_six_by_six() {
        assert_eq!(possibilities(6), 6_637_920);
    }
}