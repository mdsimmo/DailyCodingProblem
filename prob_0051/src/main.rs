/*
Given a function that generates perfectly random numbers between 1 and k (inclusive), where k is an input, write a function that shuffles a deck of cards represented as an array using only swaps.

It should run in O(N) time.

Hint: Make sure each one of the 52! permutations of the deck is equally likely.

Time to complete: 8 min
*/

extern crate rand;
use rand::Rng;

fn main() {

    let mut deck:Vec<u8> = (0..52).collect();

    let mut rnd = rand::thread_rng();
    for i in 0..deck.len() {
        let swap = rnd.gen_range(0, 52);
        deck.swap(i, swap);
    }

    println!("Suffled:\n {:?}", deck);
}