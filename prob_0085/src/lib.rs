/*
Given three 32-bit integers x, y, and b, return x if b is 1 and y if b is 0, using only mathematical
or bit operations. You can assume b can only be 1 or 0.

Time to complete: 8 min
*/

pub fn x_or_y(x: u32, y: u32, b: u32) -> u32 {
    assert!(b == 0 || b == 1);

    // make binary 1 be binary 11111...
    let mask = ((b as i32) * -1) as u32;

    // magic masking
    (x & mask) | (y & !mask)
}

#[cfg(test)]
mod tests {
    use x_or_y;

    #[test]
    fn test_x() {
        assert_eq!(42, x_or_y(42, 567, 1));
    }

    #[test]
    fn test_y() {
        assert_eq!(567, x_or_y(42, 567, 0));
    }
}