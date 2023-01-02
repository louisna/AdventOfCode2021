use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/02.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let res = lines.iter().fold(0, |score, line| {
        let line = line.as_bytes();
        score
            + (line[2] - b'W') as u64
            + ((3 + (line[2] - b'X') - (line[0] - b'A') + 1) % 3) as u64 * 3
    });

    println!("Res 1: {}", res);

    let res = lines.iter().fold(0, |score, linestr| {
        let line = linestr.as_bytes();
        let match_condition = line[2] - b'X';
        let opponent = line[0] - b'A';
        let value = ((opponent + 3 + match_condition - 1) % 3) as u64 + 1;
        score + match_condition as u64 * 3 + value
    });
    println!("Res 2: {}", res);
}
