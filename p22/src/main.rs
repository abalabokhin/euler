use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("p022_names.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut names = s.split(|c| c == '"' || c == ',').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
    names.sort();
    println!("pam {:?}", names);

    let a_code = 'A' as u32;
    let mut sum = 0;
    for x in 0..names.len() {
        sum += (x as u32 + 1) * names[x].chars().fold(0, |acc, x| acc + (x as u32 - a_code + 1));
    }
    println!("{}", sum);
}
