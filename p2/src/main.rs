fn main() {
    let mut n1 = 1;
    let mut n2 = 2;
    let mut sum = 0i64;
    while n2 < 4000000 {
        if n2 % 2 == 0 {
            sum += n2;
        }
        let (t1, t2) = (n2, n1 + n2);
        n1 = t1;
        n2 = t2;
    }
    println!("{}", sum);
}
