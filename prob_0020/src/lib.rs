/**

Given two singly linked lists that intersect at some point, find the intersecting node. The lists are non-cyclical.

For example, given A = 3 -> 7 -> 8 -> 10 and B = 99 -> 1 -> 8 -> 10, return the node with value 8.

In this example, assume nodes with the same value are the exact same node objects.

Do this in O(M + N) time (where M and N are the lengths of the lists) and constant space.


Solution:
    Just iterate by index until we reach the same value

Time to complete: 35 min (while watching YouTube :P)

*/

pub fn intersection<I, T>(mut a: I, mut b: I) -> Option<(usize, usize)>
    where I: ExactSizeIterator<Item=T>,
        T: PartialEq {

    let mut index = (0, 0);

    // Where they join, the lists must have even end length, so make the lists
    // the same tail length
    while a.len() > b.len() {
        a.next();
        index.0 += 1;
    }
    while b.len() > a.len() {
        b.next();
        index.1 += 1;
    }

    while let Some(value_a) = a.next() {
        match b.next() {
            Some(value_b) => {
                if value_a == value_b {
                    return Some(index);
                } else {
                    index.0 += 1;
                    index.1 += 1;
                }
            },
            None => return None,
        }
    };
    None
}

#[cfg(test)]
mod test {
    use intersection;

    #[test]
    fn test_given() {
        let a = [ 3, 7, 8, 10];
        let b = [99, 1, 8, 10];
        assert_eq!(intersection(a.iter(), b.iter()), Some((2, 2)));
    }

    #[test]
    fn test_different_lengths() {
        let a = [5, 6, 7, 3, 7, 8, 10];
        let b =         [99, 1, 8, 10];
        assert_eq!(intersection(a.iter(), b.iter()), Some((5, 2)));
    }

    #[test]
    fn test_no_intersection() {
        let a = [5, 6, 7, 3, 7, 9, 11];
        let b =         [99, 1, 8, 10];
        assert_eq!(intersection(a.iter(), b.iter()), None);
    }

    #[test]
    fn test_empty_list() {
        let a = [5, 6];
        let b = [];
        assert_eq!(intersection(a.iter(), b.iter()), None);
    }

    #[test]
    fn both_empty() {
        let a: [i32; 0] = [];
        let b: [i32; 0] = [];
        assert_eq!(intersection(a.iter(), b.iter()), None);
    }

}