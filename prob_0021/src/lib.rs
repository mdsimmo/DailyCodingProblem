/**

Given an array of time intervals (start, end) for classroom lectures (possibly overlapping), find the minimum number of rooms required.

For example, given [(30, 75), (0, 50), (60, 150)], you should return 2.


Time to complete: 30min

*/


pub fn min_rooms(mut times: Vec<(i32, i32)>) -> u32 {
    times.sort();

    let mut rooms_in_use:Vec<(i32, i32)> = vec![];
    let mut max_rooms = 0;
    let mut times_remaining = times.as_mut_slice();

    while let Some((&mut time_first, times_last)) = times_remaining.split_first_mut() {
        times_remaining = times_last;
        rooms_in_use.push(time_first);
        rooms_in_use.retain(|(_, e)| e > &time_first.0);
        max_rooms = std::cmp::max(max_rooms, rooms_in_use.len() as u32)
    }
    max_rooms
}


#[cfg(test)]
mod test {
    use min_rooms;

    #[test]
    fn test_given() {
        let input = vec![(30, 75), (0, 50), (60, 150)];

        assert_eq!(min_rooms(input), 2);
    }

    #[test]
    fn test_empty() {
        let input = vec![];

        assert_eq!(min_rooms(input), 0);
    }
}