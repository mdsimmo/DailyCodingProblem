/*

We're given a hashmap associating each courseId key with a list of courseIds values, which
represents that the prerequisites of courseId are courseIds. Return a sorted ordering of courses
such that we can finish all courses.

Return null if there is no such ordering.

For example, given {'CSC300': ['CSC100', 'CSC200'], 'CSC200': ['CSC100'], 'CSC100': []},
should return ['CSC100', 'CSC200', 'CSCS300'].

Time to complete: 1h 10m
*/

use std::collections::{HashMap, HashSet};

pub fn ordering<'a>(input: &HashMap<&'a str, Vec<&'a str>>) -> Option<Vec<&'a str>> {

    let mut order = vec![];
    let mut adding = HashSet::new();

    for subj in input.keys() {
        if append(input, &mut order, subj, &mut adding).is_err() {
            return None;
        }
    }

    Some(order)
}

fn append<'a>(
            input: &HashMap<&'a str, Vec<&'a str>>,
            order: &mut Vec<&'a str>, subject: &'a str,
            adding: &mut HashSet<&'a str>) -> Result<(), ()> {

    // Check that there's no problem with recursive dependencies
    if adding.contains(subject) {
        if order.contains(&subject) {
            return Result::Ok(());
        } else {
            return Result::Err(());
        }
    }
    adding.insert(subject);

    // Add the dependencies
    if let Some(dependencies) = input.get(subject) {
        for &dependency in dependencies {
            if append(input, order, dependency, adding).is_err() {
                return Result::Err(())
            }
        }
    }

    // Add its self
    order.push(subject);
    Result::Ok(())
}

#[cfg(test)]
mod tests {
    use ordering;
    use std::collections::HashMap;

    fn verify(input: &HashMap<&str, Vec<&str>>, order: &Vec<&str>) {
        for (&subj, dependencies) in input.iter() {
            let subj_index = order.iter().position(|&s| s == subj);
            assert!(subj_index.is_some());

            for &dep in dependencies {
                let dep_index = order.iter().position(|&s| s == dep);
                assert!(dep_index.is_some());
                assert!(dep_index < subj_index);
            }
        }
    }

    #[test]
    fn test_given() {
        let mut dependencies = HashMap::<&str, Vec<&str>>::new();
        dependencies.insert("CSC300", vec!["CSC100", "CSC200"]);
        dependencies.insert("CSC200", vec!["CSC100"]);
        dependencies.insert("CSC100", vec![]);

        verify(&dependencies, &ordering(&dependencies).unwrap())
    }

    #[test]
    fn test_complex() {
        let mut dependencies = HashMap::<&str, Vec<&str>>::new();
        dependencies.insert("A", vec!["B", "C"]);
        dependencies.insert("B", vec!["D"]);
        dependencies.insert("C", vec!["D", "E"]);
        dependencies.insert("F", vec!["C"]);
        dependencies.insert("G", vec!["F"]);

        let order = ordering(&dependencies);
        verify(&dependencies, &order.unwrap());
    }

    #[test]
    fn test_impossible() {
        let mut dependencies = HashMap::<&str, Vec<&str>>::new();
        dependencies.insert("A", vec!["B"]);
        dependencies.insert("B", vec!["A"]);

        let order = ordering(&dependencies);
        assert_eq!(None, order);
    }
}