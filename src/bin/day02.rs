use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/input02.txt").expect("Ewa t'es teubé Louis");
    let reader = BufReader::new(file);

    let mut depth: u64 = 0;
    let mut aim: u64 = 0;
    let mut hor: u64 = 0;
    for line in reader.lines() {
        let s = line.unwrap();
        let t: Vec<&str> = s.split(" ").collect();
        let v: u64 = t[1].parse::<u64>().unwrap();
        match t[0] {
            "up" => aim -= v,
            "down" => aim += v,
            "forward" => {
                hor += v;
                depth += aim * v;
            }
            _ => panic!("Wrong direction"),
        };
    }
    println!("{}", depth * hor);
}
