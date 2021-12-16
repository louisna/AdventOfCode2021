use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use std::collections::BinaryHeap;

fn main() {
    let file = File::open("inputs/input15.txt").expect("Lul");
    let reader = BufReader::new(file);

    let values: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    
    let mut m = values.len();
    let mut n = values[0].len();

    // Construct the BIG map
    let mut map: Vec<Vec<i32>> = vec![vec![0; n * 5]; m * 5];
    for ri in 0..5 {
        for rj in 0..5 {
            for i in 0..m {
                for j in 0..n {
                    map[ri * m + i][rj * n + j] = (values[i][j] + ri as i32 + rj as i32);
                    if map[ri * m + i][rj * n + j] > 9 {
                        map[ri * m + i][rj * n + j] -= 9;
                    }
                }
            }
        }
    }

    // Update sizes
    m *= 5;
    n *= 5;

    let mut heap = BinaryHeap::new();

    let mut visited = vec![vec![false; m]; n];

    heap.push((0, (0, 0)));
    while !heap.is_empty() {
        let (val, (i, j)) = heap.pop().unwrap();
        visited[i][j] = true;

        if i == m-1 && j == n-1 {
            println!("Val: {}", -val);
            break;
        }

        // Add all others
        if i < n-1 {
            if !visited[i+1][j] {
                heap.push((val - map[i+1][j], (i+1,j)));
                visited[i+1][j] = true;
            }
        }
        if i > 0 {
            if !visited[i-1][j] {
                heap.push((val - map[i-1][j], (i-1,j)));
                visited[i-1][j] = true;
            }
        }
        if j < n-1 {
            if !visited[i][j+1] {
                heap.push((val - map[i][j+1], (i, j+1)));
                visited[i][j+1] = true;
            }
        }
        if j > 0 {
            if !visited[i][j-1] {
                heap.push((val - map[i][j-1], (i,j-1)));
                visited[i][j-1] = true;
            }
        }
    }
}   