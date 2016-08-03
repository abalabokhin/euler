extern crate num;
use num::bigint::{BigUint, ToBigUint};
fn main() {
    let mut b : BigUint = ToBigUint::to_biguint(&1).unwrap();
    let mut a : BigUint = ToBigUint::to_biguint(&1).unwrap();
    let mut i = 2;
    while b.to_string().len() < 1000 {
        i += 1;
        let c = b.clone();
        b = a.clone() + b.clone();
        a = c;
    }
    println!("{}", i);
}
