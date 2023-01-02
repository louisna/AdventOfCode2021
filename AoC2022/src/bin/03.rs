use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/03.txt").unwrap();
    let mut lines: Vec<Vec<u8>> = BufReader::new(file).lines().map(|l| l.unwrap().as_bytes().to_vec()).collect();

    let res = lines.iter().fold(0, |score, line| {
        let left = line[..line.len() / 2].to_owned();
        let mut right = line[line.len() / 2..].to_owned();
        right.sort();
        let dup = left.iter().find(|c| right.binary_search(c).is_ok()).unwrap();
        score + get_score(dup)
    });

    println!("Res 1: {}", res);

    lines.iter_mut().for_each(|line| line.sort());
    let res = lines.chunks(3).fold(0, |score, group| {
        let group_val = group[0].iter().find(|c| group[1].binary_search(c).is_ok() && group[2].binary_search(c).is_ok()).unwrap();
        score + get_score(group_val)
    });

    println!("Res 2: {}", res);
}

fn get_score(dup: &u8) -> u64 {
    if *dup >= b'a' {
        (*dup - b'a' + 1) as u64
    } else {
        (*dup - b'A' + 27) as u64
    }
}
