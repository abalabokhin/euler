fn divisors_sum (x: i32) -> i32 {
    (1..x).filter(|&a| x % a == 0).fold(0, |acc, x| acc + x)
}

#[test]
fn test() {
    assert_eq!(divisors_sum(220), 284);
    assert_eq!(divisors_sum(284), 220);
}

fn main() {
    let sum = (2..10001).filter(|&x| {
        let y = divisors_sum(x);
        y <= 10000 && y < x && divisors_sum(y) == x
    }).fold(0, |acc, x| acc + x + divisors_sum(x));
    println!("{}", sum);
}
