use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("inputs/input14.txt").expect("Ewa t'es teubé Louis");
    let reader = BufReader::new(file);

    let mut template: HashMap<(char, char), char> = HashMap::new();
    let mut data = reader.lines();
    let mut input: Vec<char> = data.next().unwrap().unwrap().chars().collect();

    for line in data.skip(1) {
        let tl: String = line.unwrap();
        let first = tl.chars().nth(0).unwrap();
        let second = tl.chars().nth(1).unwrap();
        let result = tl.chars().nth(6).unwrap();
        template.insert((first, second), result);
    }

    // TODO: was done in python, but not in rust yet
}