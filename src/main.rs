const MODULUS: i64 = 2_i64.pow(16);
use crate::delta_functions::*;
use crate::sig_functions::*;
use std::fs::read;

mod delta_functions;
mod hash_calculation;
mod sig_functions;
mod tests;

fn main() {
    let data1 = read("example.txt").unwrap();
    let data2 = read("example_delta.txt").unwrap();

    //to show the difference in size, we print the initial files (in byte form), the signature of the original file (here it is example.txt), and the vec of deltas
    //note that the size of the deltas is much smaller than either of the two original files.
    println!("{:?}", data1); 
    println!();
    println!("{:?}", data2);
    println!();

    let x = compute_signature(&data1, 35);
    println!("{:?}", x);
    println!();

    let q = compute_deltas(&x, &data2, 35);
    println!("{:?}", q);
}
