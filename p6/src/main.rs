fn main() {
    let sum1 = (1..101).fold(0, |acc, x| acc + x * x);
    let sum2 = (1..101).fold(0, |acc, x| acc + x);

    println!("{}", sum2 * sum2 - sum1);
}
