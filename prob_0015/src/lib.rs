/*
Given a stream of elements too large to store in memory, pick a random element from
the stream with uniform probability.

Solution:
Iterate over the elements and at each element, give a small chance to choose that value.
The chance to pick the value needs is 1/num_elements_iterated.

Time to complete: 2h 10 min

*/

extern crate rand;
use rand::prelude::*;

pub fn rand_elem<T>(stream: T) -> Option<T::Item> where T: Iterator {

    let mut rng = thread_rng();
    let mut picked = None;
    for (i, v) in stream.enumerate() {
        if rng.gen_range(0, i+1) == 0 {
            picked = Some(v);
        }
    }
    return picked;
}

#[cfg(test)]
mod test {

    extern crate rayon;

    use rand_elem;

    #[test]
    fn test_even_distribution() {
        use self::rayon::prelude::*;
        use std::sync::{Arc, Mutex};

        // Input values must be index number for the histogram to work
        // (the rand_elem function will work fine whatever you feed it, but, how will
        // we measure results with duplicate values and nowhere to store results?)
        let input = vec![0, 1, 2, 3, 4, 5, 6];

        // run the function lots and build a histogram of the results
        let histogram = (0..100000).into_par_iter()
            .map(|_| rand_elem(input.iter()))
            .fold(|| Arc::new(Mutex::new(vec![0.0; input.len()])), |mut_hist, v| {
                let mut hist = mut_hist.lock().unwrap();
                hist[*v.unwrap() as usize] += 1.0;
                mut_hist.clone()
            })
            .reduce(|| Arc::new(Mutex::new(vec![0.0; input.len()])), |mut_hist_a, mut_hist_b| {
                let mut hist_a = mut_hist_a.lock().unwrap();
                let hist_b = mut_hist_b.lock().unwrap();
                assert_eq!(hist_a.len(), hist_b.len());
                for i in 0..hist_a.len() {
                    hist_a[i] += hist_b[i];
                }
                mut_hist_a.clone()
            });
        // unwrap the Arc<Option<Mutex<Option<_>>>>> result (yuck!)
        let mut histogram: Vec<f32> = Arc::try_unwrap(histogram).ok().unwrap().into_inner().ok().unwrap();

        // average out the values
        let avg = histogram.iter().sum::<f32>() / (histogram.len() as f32);
        for i in histogram.iter_mut() { *i /= avg };

        // ensure there is fairly even distribution
        assert_eq!(histogram.len(), input.len());
        for v in histogram {
            assert!(v > 0.95);
            assert!(v < 1.05);
        }
    }
}