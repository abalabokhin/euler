fn next_collatz(n:i64)->i64 {
    if n % 2 == 0 {n/2} else {3*n+1}
}

fn collatz_length(i:i64)->i32 {
    let mut n = i;
    let mut counter = 0;
    while n != 1 && n > 0{
        n = next_collatz(n);
        counter += 1;
    }
    counter
}

fn main() {
    let mut max_counter = 0;
    let mut best_i = 0;

    let _ = collatz_length(113383);

    for i in 1..1000000 {
        let counter = collatz_length(i);
        if counter >= max_counter {
            max_counter = counter;
            best_i = i;
        }
    }
    println!("{}", best_i);
}
