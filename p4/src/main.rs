use std::cmp;

fn is_polyndrome(x: i32) -> bool {
    let s = x.to_string();

    //The first part of the string
    let forward = s.chars().take(s.len() / 2);

    // The second part of the string in reverse order
    let reverse = s.chars().rev().take(s.len() / 2);

    // We group the two parts of the string in tuples
    let mut both_directions = forward.zip(reverse);

    // The word is a palindrome if each tuple contains two times
    // the same character
    both_directions.all(|(a, b)| a == b)
}

#[test]
fn test() {
    assert!(is_polyndrome(1));
    assert!(is_polyndrome(12321));
    assert!(is_polyndrome(112211));
    assert!(is_polyndrome(1));
    assert!(!is_polyndrome(21));
    assert!(is_polyndrome(1));
}

fn main() {
    let mut max = 0;
    for n1 in 100..1000 {
        for n2 in n1..1000 {
            if is_polyndrome(n1 * n2) {
                max = cmp::max(max, n1 * n2);
            }
        }
    }
    println!("{}", max);
}

