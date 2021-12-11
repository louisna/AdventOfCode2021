use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputs/input11.txt").expect("Lul");
    let reader = BufReader::new(file);

    let mut values: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let m = values.len();
    let n = values[0].len();
    let mut total_until_100 = 0;
    let mut iter = 0;

    loop {
        let mut total_this = 0;
        let mut flashed: Vec<bool> = vec![false; m * n];
        // Visit each node once at least
        values = values
            .iter()
            .map(|row| row.iter().map(|col| col + 1).collect())
            .collect();

        // Visit flashing nodes with a Queue
        let mut queue: Vec<(usize, usize)> = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if values[i][j] > 9 {
                    flashed[i * n + j] = true;
                    total_until_100 += 1;
                    total_this += 1;
                    values[i][j] = 0; // Reset next iteration
                    add_queue(&mut queue, i, j, m, n);
                }
            }
        }

        // Visit the queue to propagate flash
        while !queue.is_empty() {
            let (i, j) = queue.pop().unwrap();
            if flashed[i * n + j] {
                continue; // Already flashed node
            }
            values[i][j] += 1;
            if values[i][j] > 9 {
                values[i][j] = 0;
                flashed[i * n + j] = true;
                total_until_100 += 1;
                total_this += 1;
                add_queue(&mut queue, i, j, m, n);
            }
        }
        iter += 1;
        if iter == 100 {
            println!("{}", total_until_100);
            total_until_100 = 0; // Not usefull anymore
        }
        if total_this == m * n {
            println!("All flashed together at {}", iter);
            break;
        }
    }
}

fn add_queue(q: &mut Vec<(usize, usize)>, i: usize, j: usize, m: usize, n: usize) {
    if i > 0 {
        q.push((i - 1, j));
    }
    if i + 1 < m {
        q.push((i + 1, j));
    }
    if j > 0 {
        q.push((i, j - 1));
    }
    if j + 1 < n {
        q.push((i, j + 1));
    }
    if i > 0 && j > 0 {
        q.push((i - 1, j - 1));
    }
    if i > 0 && j + 1 < n {
        q.push((i - 1, j + 1));
    }
    if i + 1 < m && j > 0 {
        q.push((i + 1, j - 1));
    }
    if i + 1 < m && j + 1 < n {
        q.push((i + 1, j + 1));
    }
}
