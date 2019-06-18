/*
You run an e-commerce website and want to record the last N order ids in a log. Implement a data structure to accomplish this, with the following API:

    record(order_id): adds the order_id to the log
    get_last(i): gets the ith last element from the log. i is guaranteed to be smaller than or equal to N.

You should be as efficient with time and space as possible.

Solution:
Use a ring list (aka cyclic list)

*/

// Size of a RingList (rust seriously needs constant generics!!)
const N: usize = 4;

#[derive(Debug)]
pub struct RingList<T> {
    elements: [Option<T>; N],
    len: usize,
    start: usize,
}

impl <T> RingList<T> {
    pub fn new() -> Self {
        Self {
            elements: Default::default(),
            len: 0,
            start: 0,
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i >= self.len {
            None
        } else {
            self.elements[(i + self.start) % self.len].as_ref()
        }
    }

    pub fn record(&mut self, elem: T) {
        if self.len >= N {
            self.start += 1;
        } else {
            self.len += 1;
        }
        self.elements[(self.len + self.start - 1) % self.len] = Some(elem);
    }

    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn last(&self) -> Option<&T> {
        self.get(self.len-1)
    }
}

#[cfg(test)]
mod test {
    use RingList;

    #[test]
    fn test_one_insert() {
        let mut list = RingList::new();
        list.record(2);

        assert_eq!(list.get(0), Some(&2));
    }

    #[test]
    fn test_insert_several() {
        let mut list = RingList::new();

        for i in 0..4 {
            list.record(i);
        }

        // N is 4, so should only be 3 elements
        assert_eq!(list.get(3), Some(&3));
        assert_eq!(list.get(2), Some(&2));
        assert_eq!(list.get(1), Some(&1));
        assert_eq!(list.get(0), Some(&0));
    }

    #[test]
    fn test_insert_n_plus_one() {
        let mut list = RingList::new();

        for i in 0..=4 {
            list.record(i);
        }

        // N is 4, so should only be 3 elements
        assert_eq!(list.get(3), Some(&4));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(0), Some(&1));
    }


    #[test]
    fn test_lots_insert() {
        let mut list = RingList::new();

        for i in 10..=50 {
            list.record(i);
        }

        // N is 4, so should only be 3 elements
        assert_eq!(list.get(3), Some(&50));
        assert_eq!(list.get(2), Some(&49));
        assert_eq!(list.get(1), Some(&48));
        assert_eq!(list.get(0), Some(&47));
    }

    #[test]
    fn test_first_last() {
        let mut list = RingList::new();

        for i in 5..=25 {
            list.record(i);
        }

        assert_eq!(list.last(), Some(&25));
        assert_eq!(list.first(), Some(&22));
    }

    #[test]
    fn get_out_of_bounds() {
        let mut list = RingList::new();

        assert_eq!(list.get(0), None);

        for i in 0..5 {
            list.record(i);
        }

        assert_eq!(list.get(4), None);
        assert_eq!(list.get(6), None);
    }
}