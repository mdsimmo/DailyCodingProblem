use std::collections::HashSet;

fn any_two_adds_to_target(input: &Vec<i32>, target: i32) -> bool {

    let mut result = false;
    let mut needed = HashSet::with_capacity(input.len());
    for v in input {
        if needed.contains(v) {
            result = true;
            break;
        } else {
            needed.insert(target-*v);
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use any_two_adds_to_target;

    #[test]
    fn test_yes() {
        let input = vec!(10, 15, 3, 7);
        let target = 18; // 15+3

        assert!(any_two_adds_to_target(&input, target))
    }

    #[test]
    fn test_no() {
        let input = vec!(10, 15, 2, 7);
        let target = 18;

        assert!(!any_two_adds_to_target(&input, target))
    }

    #[test]
    fn test_cannot_resuse() {
        let input = vec!(10, 9, 7);
        let target = 18; // 9+9 (should fail from reuse)

        assert!(!any_two_adds_to_target(&input, target))
    }

    #[test]
    fn test_can_use_two_of_same() {
        let input = vec!(10, 9, 9, 7);
        let target = 18; // 9 + 9 (two nines in input so okay)

        assert!(any_two_adds_to_target(&input, target))
    }
}