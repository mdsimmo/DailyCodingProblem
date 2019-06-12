extern crate prob_0010;

use prob_0010::schedule;

fn main() {
    println!("Starting");

    let t1 = schedule(|| "1", 1000);
    let t2 = schedule(|| "2", 2000);
    let t3 = schedule(|| "3", 3000);
    let t4 = schedule(|| "4", 4000);
    let t5 = schedule(|| "5", 5000);

    println!("{}", t1.join().unwrap());
    println!("{}", t2.join().unwrap());
    println!("{}", t3.join().unwrap());
    println!("{}", t4.join().unwrap());
    println!("{}", t5.join().unwrap());

}