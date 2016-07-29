fn main() {
    const SIZE : usize = 21;

    let mut ways_to_end = [[0u64; SIZE]; SIZE];
    for i in 0..SIZE {
        ways_to_end[0][i] = 1;
        ways_to_end[i][0] = 1;
    }

    for i in 1..SIZE {
        for j in i..SIZE {
            ways_to_end[i][j] = ways_to_end[i-1][j] + ways_to_end[i][j-1];
            ways_to_end[j][i] = ways_to_end[j-1][i] + ways_to_end[j][i-1];
        }
    }

    println!("{:?}", ways_to_end);
}
