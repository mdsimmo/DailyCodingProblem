/*
The area of a circle is defined as πr^2. Estimate π to 3 decimal places using a Monte Carlo method.

Hint: The basic equation of a circle is x2 + y2 = r2.

Time to complete: 1h 14min
*/

extern crate rayon;
use rayon::prelude::*;

pub fn pi() -> f32 {
    // The area of on quadrant of a circle:
    // area = 1 / 4 * pi * r ^ 2;
    // thus, pi = area * 4 / r ^ 2
    // thus, if r = 2, pi = area

    let r = 2.0;
    let steps = 100;
    // Sum area in top right quadrant by summing rectangle method
    return (0..steps)
        .into_par_iter()
        // convert i to x (x=i*dx)
        .map(|i| i as f32 * r / steps as f32)
        // convert x to y by moving y down from r until y is smaller than needed.
        // We could also use y=sqrt(r^2-x^2), but I feel that's cheating; it
        // also results in many more iterations being needed to get same accuracy
        // (no idea why - maybe just coincident?)
        .map(|x| (0..=steps)
            .rev()
            .map(|y| y as f32 * r / steps as f32 )
            .filter(|y| x*x+y*y <= r*r)
            .next()
            .unwrap_or(0.0)
        )
        // convert y to rectangle area (a=y*dx)
        .map(|y| y * r / steps as f32 )
        // sum rectangle areas
        .sum();
}


#[cfg(test)]
mod test {
    use pi;

    #[test]
    fn test_3dp() {
        assert_eq!((pi() * 1000.0) as i32, 3141);
    }
}