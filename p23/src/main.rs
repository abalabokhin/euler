use std::collections::HashSet;

fn is_abundant(x: i32) -> bool {
    (1..x).filter(|a| x % a == 0).fold(0, |acc, y| acc + y) > x
}

#[test]
fn test() {
    assert!(is_abundant(12));
    assert!(!is_abundant(11));
}

fn main() {
    let abundant_numbers = (1..28124).filter(|&x| is_abundant(x)).collect::<Vec<_>>();
    let mut sum_numbers = HashSet::new();
    for x in &abundant_numbers {
        for y in &abundant_numbers {
            sum_numbers.insert(x + y);
        }
    }
    let sum = (1..28124).filter(|x| !sum_numbers.contains(x)).fold(0, |acc, x| acc+x);
    println!("{}", sum);
}


