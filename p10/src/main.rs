fn is_prime(x: i64)->bool {
    if x >= 2 {!(2..(x as f64).sqrt().floor() as i64 + 1).any(|y| x % y == 0)} else {false}
}

fn main() {
    let sum:i64 = (2..2000000).filter(|&x| is_prime(x)).fold(0, |acc, y| acc+y);
    println!("{}", sum);
}
