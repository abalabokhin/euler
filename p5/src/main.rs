fn main() {
    let num = (1..)
        .filter(|&x| (1..21).all(|a| x % a == 0)).take(1).next().unwrap();

    println!("{}", num);
}
