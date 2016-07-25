fn main() {
    let sum = (0..1000).fold(0, |acc, x| acc + x);
    println!("{}", sum);
}
