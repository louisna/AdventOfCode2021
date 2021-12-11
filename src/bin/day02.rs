use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward,
    Up,
    Down,
}

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

fn moche() {
    let values = read_file("inputs/input02.txt");
    let mut depth: u64 = 0;
    let mut f: u64 = 0;
    let mut aim: u64 = 0;
    for val in values {
        match val.0 {
            Direction::Down => aim += val.1 as u64,
            Direction::Up => aim -= val.1 as u64,
            Direction::Forward => {
                f += val.1 as u64;
                depth += aim * val.1 as u64;
            }
        }
    }
    println!("{}", depth * f);
}

fn read_file(filepath: &str) -> Vec<(Direction, u16)> {
    let file = File::open(filepath).expect("Ewa t'es teubé Louis");
    let reader = BufReader::new(file);

    let values: Vec<(Direction, u16)> = reader
        .lines()
        .map(|line| {
            let s1 = line.unwrap();
            let s: Vec<&str> = s1.split(" ").collect();
            let d: Direction = match s[0] {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => panic!("Not covered direction"),
            };
            (d, s[1].parse::<u16>().unwrap())
        })
        .collect();
    values
}
