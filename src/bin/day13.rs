use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let file = File::open("inputs/input13.txt").expect("Lul");
    let reader = BufReader::new(file);

    let mut m: usize = 0;
    let mut n: usize = 0;
    let mut stack_of_values: Vec<(usize, usize)> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        if l.len() == 0 {
            break;
        }
        let s: Vec<&str> = l.split(",").collect();
        let m1 = s[1].parse::<usize>().unwrap();
        let n1 = s[0].parse::<usize>().unwrap();
        m = if m < m1 {m1} else {m};
        n = if n < n1 {n1} else {n};
        stack_of_values.push((m1, n1));
    }
    m = m + 1;
    n = n + 1;
    
    // Now create the map
    let mut paper: Vec<Vec<bool>> = vec![vec![false; n]; m];
    for (i, j) in stack_of_values {
        paper[i][j] = true;
    }

    // Now fold the paper
    let file = File::open("inputs/input13.txt").expect("Lul");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        if !l.contains("fold") {
            continue;
        }
        //print_paper(&paper, m, n);
        let s: Vec<&str> = l.split(" ").collect();
        let dirv: Vec<&str> = s[2].split("=").collect();
        let v = dirv[1].parse::<usize>().unwrap();
        match dirv[0] {
            "y" => {
                fold_hor(&mut paper, v, m, n);
                m = v;
            },
            _ => {
                fold_ver(&mut paper, v, m, n);
                n = v;
            }
        }
        println!("Remaining points: {}", count_true(&paper, m, n));
    }
    print_paper(&paper, m, n);
}

fn count_true(paper: &Vec<Vec<bool>>, m: usize, n: usize) -> u32 {
    let mut total = 0;
    for i in 0..m {
        for j in 0..n {
            match paper[i][j] {
                true => total += 1,
                _ => (),
            }
        }
    }
    total
}

fn fold_hor(paper: &mut Vec<Vec<bool>>, v: usize, m: usize, n: usize) {
    for i in v + 1..m {
        for j in 0..n {
            let d = i - v;
            paper[v - d][j] |= paper[i][j];
        }
    }
}

fn fold_ver(paper: &mut Vec<Vec<bool>>, v: usize, m: usize, n: usize) {
    for i in 0..m {
        for j in v + 1..n {
            let d = j - v;
            paper[i][v - d] |= paper[i][j];
        }
    }
}

fn print_paper(paper: &Vec<Vec<bool>>, m: usize, n: usize) {
    for i in 0..m {
        for j in 0..n {
            let v = if paper[i][j] {"x"} else {"."};
            print!("{}", v);
        }
        println!("");
    }
}