use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/input16.txt").expect("Lul");
    let reader = BufReader::new(file);

    let packet: Vec<char> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(convert_to_binary)
                .collect::<Vec<&str>>()
                .join("")
        })
        .collect::<Vec<String>>()[0].to_owned().chars().collect();
    println!("{:?}\n\n", packet);

    println!("{:?}", parse_packet(&packet));
}

fn parse_packet(packet: &[char]) -> (u64, usize, u64) {
    let mut total = 0;
    let mut value: u64 = 0;
    let mut idx: usize = 0;

    // First three bits are the version
    total += char_to_val(&packet[0..3]);
    idx += 3;
    //println!("version: {}", total);
    let p_type = char_to_val(&packet[idx..idx + 3]);
    idx += 3;
    //println!("ptype: {}", p_type);
    if p_type == 4 { // Literal
        let tup = literal_value(&packet[idx..]);
        idx += tup.1;
        value = tup.0 as u64;
    } else { // Operand
        let mut sub_values: Vec<u64> = Vec::new();
        // Find size
        if packet[idx] == '0' {
            idx += 1;
            // Parse 15 following bits
            let length: usize = char_to_val(&packet[idx..idx + 15]) as usize;
            //println!("Taille 0: length={}", length);
            idx += 15;
            let mut current_length: usize = 0;
            while current_length < length {
                let tup2 = parse_packet(&packet[idx..]);
                current_length += tup2.1;
                total += tup2.0;
                idx += tup2.1;
                sub_values.push(tup2.2);
            }
        } else {
            idx += 1;
            let nb_packet = char_to_val(&packet[idx..idx + 11]);
            //println!("Taille 1: nb_packets={}", nb_packet);
            idx += 11;
            for _ in 0..nb_packet {
                let tup2 = parse_packet(&packet[idx..]);
                total += tup2.0;
                idx += tup2.1;
                sub_values.push(tup2.2);
            }
        }
        value += compute_value(&sub_values, p_type);
    }

    (total, idx, value)
}

fn compute_value(values: &[u64], p_type: u64) -> u64 {
    match p_type {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => if values[0] > values[1] { 1 } else { 0 },
        6 => if values[0] < values[1] { 1 } else { 0 },
        7 => if values[0] == values[1] { 1 } else { 0 },
        _ => panic!("Unsupported operation"),
    }
}

fn char_to_val(c: &[char]) -> u64 {
    c.iter().fold(0, |score, car| {
        (score << 1) + car.to_digit(2).unwrap() as u64
    })
}

fn literal_value(packet: &[char]) -> (u64, usize) {
    let mut value: u64 = 0;
    let mut idx: usize = 0;
    
    loop {
        let last_chunk = packet[idx] == '0';
        idx += 1;
        value = (value << 4) + char_to_val(&packet[idx..idx + 4]);
        idx += 4;
        if last_chunk {
            break;
        }
    }
    (value, idx)
}

fn convert_to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}