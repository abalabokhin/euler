extern crate num;
use num::bigint::{BigUint, ToBigUint};
fn main() {
    let a : BigUint = ToBigUint::to_biguint(&2).unwrap();
    let mut b = a.clone();
    for _ in 1..1000 {
        b = b * a.clone();
    }
    let s = b.to_string();
    let acc = s.chars().fold(0, |acc, x| acc + x.to_digit(10).unwrap());
    println!("{} {}", b.to_string(), acc);
}
