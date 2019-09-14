/*

Given a number in the form of a list of digits, return all possible permutations.

For example, given [1,2,3], return [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

Time to complete: 24m (was watching movie at same time though)
*/

use std::collections::HashSet;

pub fn permutations(input: &[i32]) -> HashSet<Vec<i32>> {
    if input.len() == 0 {
        return HashSet::new();
    } else if input.len() == 1 {
        let mut result = HashSet::new();
        result.insert(vec![input[0]]);
        return result;
    }

    let mut all_perms = HashSet::new();

    for i in 0..input.len() {
        let mut others = vec![];
        for j in 0..input.len() {
            if i != j {
                others.push(input[j]);
            }
        }

        let subperms = permutations(&others[..]);
        for mut perm in subperms.into_iter() {
            perm.insert(0, input[i]);
            all_perms.insert(perm);
        }
    }

    all_perms
}

#[cfg(test)]
mod tests {
    use permutations;
    use std::collections::HashSet;

    #[test]
    fn test_impossible() {
        let expected: HashSet<_> = vec![
            vec![1, 2, 3], vec![1, 3, 2],
            vec![2, 1, 3], vec![2, 3, 1],
            vec![3, 2, 1], vec![3, 1, 2]
        ].into_iter().collect();

        assert_eq!(expected, permutations(&[1, 2, 3]));
    }
}