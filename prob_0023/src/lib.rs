/**
You are given an M by N matrix consisting of booleans that represents a board. Each True boolean
represents a wall. Each False boolean represents a tile you can walk on.

Given this matrix, a start coordinate, and an end coordinate, return the minimum number of steps
required to reach the end coordinate from the start. If there is no possible path, then return null.
 You can move up, left, down, and right. You cannot move through walls. You cannot wrap around the
 edges of the board.

For example, given the following board:

[[f, f, f, f],
[t, t, f, t],
[f, f, f, f],
[f, f, f, f]]

and start = (3, 0) (bottom left) and end = (0, 0) (top left), the minimum number of steps required
 to reach the end is 7, since we would need to go through (1, 2) because there is a wall everywhere
 else on the second row.


Time to complete: 2h 30min

*/

#[cfg_attr(test, macro_use)]
extern crate ndarray;

use ndarray::prelude::*;
use std::ops::Add;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl <'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, rhs: &'b Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Moves> for &Point {
    type Output = Point;

    fn add(self, rhs: &Moves) -> Self::Output {
        let other: Point = (*rhs).into();
        self.add(&other)
    }
}

struct Node {
    pre: Option<Rc<Node>>,
    loc: Point,
}

impl Node {
    fn iter(&self) -> Iter {
        Iter {
            node: Some(&self),
        }
    }
}

struct Iter<'a> {
    node: Option<&'a Node>,
}

impl <'a> Iterator for Iter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node;

        self.node = match self.node {
            Some(v) => match &v.pre {
                Some(n) => { Some(n) },
                None => { None },
            },
            None => None,
        };

        current
    }
}

#[derive(Copy, Clone)]
enum Moves {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Moves {
    const ALL:&'static [Moves] = &[Moves::LEFT, Moves::RIGHT, Moves::UP, Moves::DOWN];
}

impl Into<Point> for Moves {
    fn into(self) -> Point {
        match self {
            Moves::LEFT  => Point{x: -1, y:  0},
            Moves::RIGHT => Point{x:  1, y:  0},
            Moves::UP    => Point{x:  0, y: -1},
            Moves::DOWN  => Point{x:  0, y:  1},
        }
    }
}

pub fn shortest_path(start: Point, end: Point, grid: &Array2<bool>) -> Option<usize> {

    // initialise the frontier
    let mut frontier = VecDeque::new();
    frontier.push_front(Node { pre: None, loc: start } );

    while let Some(next) = frontier.pop_front() {
        if next.loc == end {
            return Some(next.iter().count()-1)
        }

        let next = Rc::new(next);

        for mv in Moves::ALL {
            let p = &next.loc + mv;

            // test for any walls (is there are nicer way to do this?)
            if p.x >=0 && p.y >= 0 {
                if let Some(&wall) = grid.get((p.y as usize, p.x as usize)) {
                    if wall {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }

            // Repeat state check
            if next.iter().any(|n| n.loc == p) {
                continue;
            }

            // push the new search location
            frontier.push_back(Node {pre: Some(next.clone()), loc: p })
        }
    }

    None
}


#[cfg(test)]
mod test {
    use ::*;

    #[test]
    fn test_maze() {
        let f = false;
        let t = true;
        let grid = array![
                [f, f, f, f],
                [t, t, f, t],
                [f, f, f, f],
                [f, f, f, f]
        ];
        let start = Point {x: 0, y: 3};
        let end = Point {x: 0, y: 0 };

        assert_eq!(shortest_path(start, end, &grid), Some(7));
    }


    #[test]
    fn test_impossible() {
        let f = false;
        let t = true;
        let grid = array![
                [f, f, f, f],
                [t, t, t, t],
                [f, f, f, f]
        ];
        let start = Point {x: 0, y: 2};
        let end = Point {x: 0, y: 0 };

        assert_eq!(shortest_path(start, end, &grid), None);
    }
}