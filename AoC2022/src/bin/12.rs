use std::collections::LinkedList;

fn main() {
    let content = include_str!("../../inputs/12.txt");
    let mut map: Vec<_> = content.split('\n').map(|line| line.as_bytes().to_vec()).collect();
    let m = map.len();
    let n = map[0].len();
    
    let mut end = (0, 0);
    // Should use "find" but flemme.
    for i in 0..m {
        for j in 0..n {
            if map[i][j] == b'S' {
                map[i][j] = b'a';
            } else if map[i][j] == b'E' {
                end = (i, j);
                map[i][j] = b'z';
            }
        }
    }

    let mut all_costs = Vec::new();
    
    for i in 0..m {
        'intern_loop: for j in 0..n {
            if map[i][j] == b'a' {

                let mut queue = LinkedList::new();
                queue.push_back((i, j, 0));
                let mut visited = vec![vec![false; n]; m];
                while !queue.is_empty() {
                    let (i, j, cost) = queue.pop_front().unwrap();
                    
                    if visited[i][j] {
                        continue;
                    }
                    visited[i][j] = true;
            
                    if i == end.0 && j == end.1 {
                        all_costs.push(cost);
                        continue 'intern_loop;
                    }
            
                    if i > 0 && map[i - 1][j].saturating_sub(map[i][j]) <= 1 {
                        queue.push_back((i - 1, j, cost + 1));
                    }
            
                    if i < m - 1 && map[i + 1][j].saturating_sub(map[i][j]) <= 1 {
                        queue.push_back((i + 1, j, cost + 1));
                    }
            
                    if j > 0 && map[i][j - 1].saturating_sub(map[i][j]) <= 1 {
                        queue.push_back((i, j - 1, cost + 1));
                    }
            
                    if j < n - 1 && map[i][j + 1].saturating_sub(map[i][j]) <= 1 {
                        queue.push_back((i, j + 1, cost + 1));
                    }
                }
                
            }
        }
    }

    all_costs.sort();
    println!("Res 2: {}", all_costs[0]);
}