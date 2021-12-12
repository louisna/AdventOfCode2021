use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

fn main() {
    let file = File::open("inputs/input12.txt").expect("Lul");
    let reader = BufReader::new(file);

    let mut adj: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let split: Vec<&str> = l.split("-").collect();
        adj.entry(split[0].to_string()).or_insert(Vec::new()).push(split[1].to_string());
        adj.entry(split[1].to_string()).or_insert(Vec::new()).push(split[0].to_string());
    }
    
    // Part 1
    let nb_path = dfs("start", HashSet::new(), &adj, true);
    println!("Nb path: {}", nb_path);

    // Part 2
    let nb_path = dfs("start", HashSet::new(), &adj, false);
    println!("Nb path: {}", nb_path);

}

fn dfs(node: &str, mut visited: HashSet<String>, adj: &HashMap<String, Vec<String>>, visited_twice: bool) -> u32 {
    let mut count = 0;
    if node == "end" {
        return 1;
    }
    if node.chars().next().unwrap() != node.chars().next().unwrap().to_uppercase().next().unwrap() {
        visited.insert(node.to_string());
    }
    for neigh in adj.get(&node.to_string()).unwrap() {
        if !visited.contains(neigh) {
            count += dfs(neigh, visited.clone(), adj, visited_twice);
        } else if !visited_twice && neigh != "start" {
            count += dfs(neigh, visited.clone(), adj, true);
        }
    }
    count
}