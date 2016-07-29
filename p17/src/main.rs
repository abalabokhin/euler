fn letters_number(x : i32) -> Option<usize> {
    match x {
        1 => Some("one".len()),
        2 => Some("two".len()),
        3 => Some("three".len()),
        4 => Some("four".len()),
        5 => Some("five".len()),
        6 => Some("six".len()),
        7 => Some("seven".len()),
        8 => Some("eight".len()),
        9 => Some("nine".len()),
        10 => Some("ten".len()),
        11 => Some("eleven".len()),
        12 => Some("twelve".len()),
        13 => Some("thirteen".len()),
        14 => Some("fourteen".len()),
        15 => Some("fifteen".len()),
        16 => Some("sixteen".len()),
        17 => Some("seventeen".len()),
        18 => Some("eighteen".len()),
        19 => Some("nineteen".len()),
        20 => Some("twenty".len()),
        30 => Some("thirty".len()),
        40 => Some("forty".len()),
        50 => Some("fifty".len()),
        60 => Some("sixty".len()),
        70 => Some("seventy".len()),
        80 => Some("eighty".len()),
        90 => Some("ninety".len()),
        1000 => Some("onethousand".len()),
        x if x < 100 => Some(letters_number((x / 10) * 10).unwrap() + letters_number(x % 10).unwrap()),
        x if x < 1000 && x % 100 == 0 => Some(letters_number(x / 100).unwrap() + "hundred".len()),
        x if x < 1000 => Some(letters_number(x / 100).unwrap() + "hundred".len() + "and".len() + letters_number(x % 100).unwrap()),
        _ => None,}
}

#[test]
fn test() {
    assert_eq!(Some(3), letters_number(1));
    assert_eq!(Some(5), letters_number(7));
    assert_eq!(Some(5), letters_number(7));
    assert_eq!(Some(9), letters_number(21));
    assert_eq!(Some(11), letters_number(79));
    assert_eq!(Some(12), letters_number(300));
    assert_eq!(Some(23), letters_number(342));
    assert_eq!(Some(20), letters_number(115));
    assert_eq!(Some(11), letters_number(1000));
}

fn main() {
    let sum = (1..1001).fold(0, |acc, x| acc + letters_number(x).unwrap());
    println!("{}", sum);
}
