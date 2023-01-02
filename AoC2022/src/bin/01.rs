use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/01-a.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());

    let (mut res, _) = lines.fold((Vec::new(), 0), |(mut vec, current), line| {
        if line.is_empty() {
            vec.push(current);
            (vec, 0)
        } else {
            (vec, current + line.parse::<u64>().unwrap())
        }
    });

    res.sort();

    println!("Res 1: {}", res.last().unwrap());

    println!("Res 2: {}", (&res[res.len() - 3..]).iter().sum::<u64>());
}
