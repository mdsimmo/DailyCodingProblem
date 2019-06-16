/*
Given a stream of elements too large to store in memory, pick a random element from
the stream with uniform probability.

Solution:
Iterate over the elements and at each element, give a small chance to choose that value.
The chance to pick the value needs is 1/num_elements_iterated.
*/

extern crate rand;
extern crate map_in_place;

use rand::prelude::*;
use map_in_place::MapVecInPlace;

fn rand_elem<T>(stream: T) -> Option<T::Item> where T: Iterator {

    let mut rng = thread_rng();
    let mut picked = None;
    for (i, v) in stream.enumerate() {
        if rng.gen_range(0, i+1) == 0 {
            picked = Some(v);
        }
    }
    return picked;
}

fn main() {

    let input = vec![0, 1, 2, 3, 4, 5, 6];

    let mut histogram = vec![0.0; input.len()];

    for _ in 0..1000000 {
        let v = rand_elem(input.iter());
        histogram[*v.unwrap() as usize] += 1.0;
    }

    let avg = histogram.iter().sum::<f32>() / (histogram.len() as f32);
    let histogram = histogram.map_in_place(|v| v / avg);

    println!("{:?}", histogram);

}