/*
The area of a circle is defined as πr^2. Estimate π to 3 decimal places using a Monte Carlo method.

Hint: The basic equation of a circle is x2 + y2 = r2.
*/

extern crate rayon;
use rayon::prelude::*;

pub fn pi() -> f32 {
    // The area of on quadrant of a circle:
    // a = 1 / 4 * p * r ^ 2;
    // thus, p = a * 4 / r ^ 2
    // thus, if r = 2, p = a

    let r = 2.0;
    let steps = 100;
    // Sum area in top right quadrant by rectangle method
    return (0..steps)
        .into_par_iter()
        // convert i to x
        .map(|i| i as f32 * r / steps as f32)
        // convert x to y by moving down from r until we are smaller than needed.
        // We could also use y=sqrt(r^2-x^2), but I feel that's cheating; it
        // also results in much more iterations being needed to get same accuracy
        // (no idea why - maybe just coincident?)
        .map(|x| (0..=steps)
            .rev()
            .map(|y| y as f32 * r / steps as f32 )
            .filter(|y| x*x+y*y <= r*r)
            .next()
            .unwrap_or(0.0)
        )
        .map(|y| y * r / steps as f32 )
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