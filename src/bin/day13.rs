use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let file = File::open("inputs/input13.txt").expect("Lul");
    let reader = BufReader::new(file);

    let mut m: usize = 0;
    let mut n: usize = 0;
    let mut stack_of_values: Vec<(usize, usize)> = Vec::new();
    for line_ in reader.lines() {
        let line = line_.unwrap();
        if line.is_empty() {
            break;
        }
        let s: Vec<&str> = line.split(',').collect();
        let m1 = s[1].parse::<usize>().unwrap();
        let n1 = s[0].parse::<usize>().unwrap();
        m = if m < m1 { m1 } else { m };
        n = if n < n1 { n1 } else { n };
        stack_of_values.push((m1, n1));
    }
    m += 1;
    n += 1;

    // Now create the map
    let mut paper: Vec<Vec<u32>> = vec![vec![0; n]; m];
    for (i, j) in stack_of_values {
        paper[i][j] = 1;
    }

    // Now fold the paper
    let file = File::open("inputs/input13.txt").expect("Lul");
    let reader = BufReader::new(file);
    for line_ in reader.lines() {
        let line = line_.unwrap();
        if !line.contains("fold") {
            continue;
        }
        //print_paper(&paper, m, n);
        let s: Vec<&str> = line.split(' ').collect();
        let dirv: Vec<&str> = s[2].split('=').collect();
        let v = dirv[1].parse::<usize>().unwrap();
        match dirv[0] {
            "y" => {
                fold_hor(&mut paper, v, m, n);
                m = v;
            }
            _ => {
                fold_ver(&mut paper, v, m, n);
                n = v;
            }
        }
        println!("Remaining points: {}", count_true(&paper));
    }
    print_paper(&paper, m, n);
}

fn count_true(paper: &[Vec<u32>]) -> u32 {
    paper
        .iter()
        .fold(0, |score, line| score + line.iter().sum::<u32>())
}

fn fold_hor(paper: &mut Vec<Vec<u32>>, v: usize, m: usize, n: usize) {
    for i in v + 1..m {
        for j in 0..n {
            let d = i - v;
            paper[v - d][j] |= paper[i][j];
        }
    }
}

fn fold_ver(paper: &mut Vec<Vec<u32>>, v: usize, _m: usize, n: usize) {
    for line in paper {
        for j in v + 1..n {
            let d = j - v;
            line[v - d] |= line[j];
        }
    }
}

fn print_paper(paper: &[Vec<u32>], m: usize, n: usize) {
    for line in paper.iter().take(m) {
        for val in line.iter().take(n) {
            let v = if *val == 1 { "x" } else { "." };
            print!("{}", v);
        }
        println!();
    }
}
