use std::fs::File;
use std::io::prelude::*;
use std::cmp;
fn main() {
    let mut f = File::open("input.txt").unwrap();
//    let mut f = File::open("p067_triangle.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("{}", s);

    let mut triangle = Vec::new();
    for token in s.split("\n").filter(|&x| !x.is_empty()){
        let v = token.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        triangle.push(v);
    }

    let mut best_sums = Vec::new();
    best_sums.push(vec![triangle[0][0]]);


    for raw in 1..triangle.len() {
        let mut v = vec![best_sums[raw - 1][0] + triangle[raw][0]];
        v.extend(vec![0; raw + 1 - 2]);
        v.extend(vec![best_sums[raw - 1][raw - 1] + triangle[raw][raw]]);
        best_sums.push(v);
    }

    for raw in 1..triangle.len() {
        for column in 1..raw {
            best_sums[raw][column] = cmp::max(best_sums[raw - 1][column - 1] + triangle[raw][column], best_sums[raw - 1][column] + triangle[raw][column]);
        }
    }
    println!("{:?}", best_sums);
    let max = best_sums.last().unwrap().iter().max().unwrap();

    println!("{}", max);
}
