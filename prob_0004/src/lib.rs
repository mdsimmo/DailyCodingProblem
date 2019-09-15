fn strip_negatives(input: &mut [i32]) -> &mut [i32] {
    let mut last_positive = input.len();
    let mut index = 0;
    while index < last_positive {
        let v = input[index];
        if v <= 0 {
            last_positive -= 1;
            input[index] = input[last_positive];
            input[last_positive] = v;
        } else {
            index += 1;
        }
    }

    return &mut input[0..last_positive];
}

fn mark_indexs(input: &mut [i32]) -> &[i32] {
    for i in 0..input.len() {
        let v = input[i].abs();
        input.get_mut((v-1) as usize).map(|i| *i = -i.abs());
    }
    return input;
}

fn find_first_positive(input: &[i32]) -> i32 {
    for (i, &v) in input.iter().enumerate() {
        if v > 0 {
            return (i+1) as i32;
        }
    }
    return input.len() as i32 + 1;
}

pub fn solve(input: &mut [i32]) -> i32 {
    let mut input = strip_negatives(input);
    let input = mark_indexs(&mut input);
    find_first_positive(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_prob_0004() {
        assert_eq!(1, solve(&mut [2, 3, 4, 5]));
        assert_eq!(5, solve(&mut [1, 2, 3, 4]));
        assert_eq!(2, solve(&mut [4, 6, 3, 1]));
        assert_eq!(2, solve(&mut [4, -3, -2, 1]));
        assert_eq!(4, solve(&mut [2, 2, 1, 3, 5]));
        assert_eq!(3, solve(&mut [6, 2, 9, -3, -2, 0, 1, 8, 1, 5]));
    }
}
