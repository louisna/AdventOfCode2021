use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let depths = read_file("inputs/input011.txt");
    // let depths = read_file("inputs/test_input011.txt");

    let mut summed: Vec<u16> = Vec::with_capacity(depths.len());
    for i in 0..depths.len() - 2 {
        summed.push(depths[i] + depths[i+1] + depths[i+2]);
    }
    let sum: Vec<u16> = (0..depths.len()-2).map(|i| (&depths[i..i+3]).iter().sum()).collect();

    let count: Vec<&[u16]> = sum.windows(2).filter(|w| w[0] < w[1]).collect();
    println!("{:?}", count.len());
}

/// Comes from https://stackoverflow.com/questions/65100493/how-to-read-a-list-of-numbers-from-a-file-into-a-vec
fn read_file(filepath: &str) -> Vec<u16> {
    let file = File::open(filepath).expect("Ewa t'es teubé Louis");
    let reader = BufReader::new(file);

    let numbers: Vec<u16> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u16>().unwrap())
        .collect();
    numbers
}