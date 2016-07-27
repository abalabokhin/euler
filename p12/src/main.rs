fn divisors_number (x:i64)->i32 {
    (1..(x as f64).sqrt().floor() as i64).filter(|&y| x % y == 0).count() as i32
}

fn main() {
    let mut sum = 0;
    for x in 1.. {
        sum += x;
        let n = divisors_number(sum);
        println!("{} {}", sum, n);

        if n > 500 {
            println!("{}", sum);
            break;
        }
    }
}
