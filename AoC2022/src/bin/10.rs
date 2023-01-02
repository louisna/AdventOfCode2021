use std::collections::HashMap;
use std::fmt::Write;

fn main() {
    let content = include_str!("../../inputs/10.txt");

    let mut nb_cycle: i64 = 0;
    let mut value: i64 = 1;
    let mut hash = HashMap::new();
    let mut s = String::new();
    let mut all = Vec::new();

    for line in content.split("\n") {
        let mut tab = line.split(" ");
        // println!("Tmp: {} {}... {}", nb_cycle, value, line);
        match tab.next().unwrap() {
            "noop" => {
                if nb_cycle % 20 == 0 && !hash.contains_key(&nb_cycle) {
                    hash.insert(nb_cycle, nb_cycle * value);
                }
                nb_cycle += 1;
                if nb_cycle % 20 == 0 && !hash.contains_key(&nb_cycle) {
                    hash.insert(nb_cycle, nb_cycle * value);
                }
                all.push(value);
            },
            "addx" => {
                if nb_cycle % 20 == 19 && !hash.contains_key(&(nb_cycle + 1)) {
                    hash.insert(nb_cycle + 1, (nb_cycle + 1) * value);
                } else if nb_cycle % 20 == 0 && !hash.contains_key(&nb_cycle) {
                    hash.insert(nb_cycle, nb_cycle * value);
                }
                let incr: i64 = tab.next().unwrap().parse().unwrap();
                all.push(value);
                all.push(value);
                nb_cycle += 2;
                if nb_cycle % 20 == 0 && !hash.contains_key(&nb_cycle) {
                    hash.insert(nb_cycle, nb_cycle * value);
                }
                value += incr;
            },
            _ => panic!("Unknown action"),
        }
    }

    // println!("HASH: {:?}", hash);

    let total: i64 = [20, 60, 100, 140, 180, 220].iter().map(|i| hash.get(i).unwrap()).sum();
    println!("Res 1: {}", total);

    println!("{}", s);
    println!("Values: {:?}", all);
    let m = 40;
    for (i, val) in all.iter().enumerate() {
        if (i as i64 % m) >= ((val - 1) % m) && (i as i64 % m) <= ((val + 1) % m) {
            write!(s, "#").unwrap();
        } else {
            write!(s, ".").unwrap();
        }

        if i as i64 % m == 0 {
            writeln!(s, "").unwrap();
        }
    }

    println!("{}", s);
}