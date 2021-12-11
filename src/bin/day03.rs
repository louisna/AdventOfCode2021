use std::fs::File;
use std::io::{BufRead, BufReader};

/// Comes from https://stackoverflow.com/questions/65100493/how-to-read-a-list-of-numbers-from-a-file-into-a-vec
fn main() -> () {
    let file = File::open("inputs/input03.txt").expect("Ewa t'es teubé Louis");
    let reader = BufReader::new(file);
    let nb_bits = 12;

    let mut v: Vec<i32> = vec![0; nb_bits];
    let mut numbers: Vec<u16> = Vec::with_capacity(1000);

    for line in reader.lines() {
        let s = line.unwrap();
        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => v[i] -= 1,
                '1' => v[i] += 1,
                _ => panic!("Wrong bit"),
            };
        }
        numbers.push(u16::from_str_radix(&s, 2).unwrap());
    }

    let gamma = v.iter().fold(
        0 as u32,
        |acc, x| {
            if *x > 0 {
                (acc << 1) + 1
            } else {
                acc << 1
            }
        },
    );
    let epsilon: u32 = !gamma & 0b111111111111;
    println!("{}", gamma * epsilon);

    // Second part
    let oxygen = get_status(
        &numbers,
        |x| {
            if x >= 0 {
                1
            } else {
                0
            }
        },
        nb_bits - 1,
    ) as u64;
    let co2 = get_status(
        &numbers,
        |x| {
            if x >= 0 {
                0
            } else {
                1
            }
        },
        nb_bits - 1,
    ) as u64;
    // println!("{} {}", oxygen, co2);
    println!("{}", oxygen * co2);
}

fn get_status<F>(v: &Vec<u16>, f: F, size: usize) -> u16
where
    F: Fn(i32) -> u16,
{
    // F: closure to filter
    let mut nb_remaining = v.len();
    let mut round = size as i32;
    let mut active: Vec<bool> = vec![true; v.len()];
    while nb_remaining > 1 {
        let mut count: i32 = 0;
        for (i, value) in v.iter().enumerate() {
            if !active[i] {
                continue;
            }
            // println!("VALUE {:b} avec round donne {} mais {}", value, 1 << round, value & (1 << round));
            match value & (1 << round) {
                0 => count -= 1,
                _ => count += 1,
            }
        }
        //println!("Count is {}", count);
        let bit = f(count);
        //println!("Bit is then {}", bit);
        for (i, value) in v.iter().enumerate() {
            if !active[i] {
                continue;
            } else if (value & (1 << round)) ^ (bit << round) != 0 {
                active[i] = false;
                nb_remaining -= 1;
                //println!("Remove {}", i);
            } else {
                // println!("Active number is {:b}", value);
                //println!("Number is {:b}, only its bit {}, the bit {}, the bit shifted {}", value, value & (1 << round), bit, bit << round);
            }
        }
        // println!("Round {} active {}", round, nb_remaining);
        round -= 1;
    }
    // Only one active I hope
    for (i, value) in v.iter().enumerate() {
        if active[i] {
            return *value;
        }
    }
    0
}
