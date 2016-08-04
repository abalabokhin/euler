use std::cmp;

fn main() {
    let mut n = 600851475143i64;
    let mut max_div = 0;
    loop {
        let mut local_max_div = 0;
        for x in cmp::max(2, max_div)..n {
            if n % x == 0 {
                n = n / x;
                local_max_div = x;
                break;
            }
        }
        if local_max_div == 0 {
            max_div = cmp::max(max_div, n);
            break;
        } else {
            max_div = local_max_div;
        }
    }
    println!("{}", max_div);
}
