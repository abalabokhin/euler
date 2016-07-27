fn is_prime(x: i32)->bool {
    if x >= 2 {!(2..(x as f64).sqrt().floor() as i32 + 1).any(|y| x % y == 0)} else {false}
}

#[test]
fn test() {
    assert!(!is_prime(-5));
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(13));
    assert!(is_prime(23));
    assert!(!is_prime(4));
}

fn main() {
    let prime = (1..).filter(|&x| is_prime(x)).nth(10000).unwrap();
    println!("{}", prime);
}
