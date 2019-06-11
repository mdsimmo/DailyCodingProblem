fn product_with_div(input: &Vec<i32>) -> Vec<i32> {
    let mult: i32 = input.iter().product();
    input.iter().map(|x| mult/(*x)).collect()
}

fn product_without_div(input: &Vec<i32>) -> Vec<i32> {
    input.iter().enumerate().map(
        |(i, _)| {
            let left: i32 = input.iter().take(i).product();
            let right: i32 = input.iter().skip(i+1).product();
            left * right
        }
    ).collect()
}


#[cfg(test)]
mod tests {
    use ::{product_with_div, product_without_div};

    #[test]
    fn test_with_div() {
        assert_eq!(
            product_with_div(&vec![1, 2, 3, 4, 5]),
            vec![120, 60, 40, 30, 24]
        );
    }

    #[test]
    fn test_without_div() {
        assert_eq!(
            product_without_div(&vec![1, 2, 3, 4, 5]),
            vec![120, 60, 40, 30, 24]
        );
    }
}