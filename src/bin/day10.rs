use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/input10.txt").expect("Lul");
    let reader = BufReader::new(file);

    // let scores: Vec<u64> = reader
    //     .lines()
    //     .map(|line| line_score(line.unwrap())).collect();
    // println!("{}", scores.iter().sum::<u64>());

    let mut scores2: Vec<u64> = reader
        .lines()
        .map(|line| complete(line.unwrap()))
        .filter(|&x| x > 0)
        .collect::<Vec<u64>>();
    scores2.sort();
    let med = scores2.len() / 2;
    println!("{:?}", scores2);
    println!("{}", scores2[med]);
}

fn line_score(line: String) -> u64 {
    let mut stack: Vec<char> = Vec::with_capacity(line.len());
    for c in line.chars() {
        match c {
            ']' => {
                let o = stack.pop().unwrap();
                if o != '[' {
                    return char_score(c);
                }
            }
            ')' => {
                let o = stack.pop().unwrap();
                if o != '(' {
                    return char_score(c);
                }
            }
            '>' => {
                let o = stack.pop().unwrap();
                if o != '<' {
                    return char_score(c);
                }
            }
            '}' => {
                let o = stack.pop().unwrap();
                if o != '{' {
                    return char_score(c);
                }
            }
            _ => stack.push(c),
        }
    }
    0
}

fn char_score(c: char) -> u64 {
    match c {
        '}' => 1197,
        ')' => 3,
        ']' => 57,
        _ => 25137,
    }
}

fn complete(line: String) -> u64 {
    let mut stack: Vec<char> = Vec::with_capacity(line.len());
    for c in line.chars() {
        match c {
            ']' | ')' | '}' | '>' => match stack.pop().unwrap() {
                '(' if c == ')' => (),
                '[' if c == ']' => (),
                '{' if c == '}' => (),
                '<' if c == '>' => (),
                _ => return 0,
            },
            _ => stack.push(c),
        }
    }
    // println!("{:?}", stack);
    stack.iter().rev().fold(0, |score, c| match c {
        '[' => score * 5 + 2,
        '(' => score * 5 + 1,
        '{' => score * 5 + 3,
        '<' => score * 5 + 4,
        _ => panic!("Not covered"),
    })
}
