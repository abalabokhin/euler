fn main() {
    let sum = (0..1000).filter(|x| x % 3 == 0 || x % 5 == 0).fold(0, |acc, x| acc + x);
    println!("{}", sum);
}
