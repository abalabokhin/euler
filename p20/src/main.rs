extern crate num;
use num::bigint::{BigUint, ToBigUint};
fn main() {
    let mut b : BigUint = ToBigUint::to_biguint(&1).unwrap();
    for x in 1..101 {
        let a : BigUint = ToBigUint::to_biguint(&x).unwrap();
        b = b * a;
    }
    let s = b.to_string();
    let acc = s.chars().fold(0, |acc, x| acc + x.to_digit(10).unwrap());
    println!("{} {}", b.to_string(), acc);
}
