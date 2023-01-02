use std::collections::HashSet;

fn main() {
    let content = include_str!("../../inputs/test.txt");
    let cubes: HashSet<Vec<i32>> = HashSet::from_iter(content.split('\n').map(|line| line.split(',').map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>()));
    println!("Cubes: {:?}", cubes);

    let total = cubes.iter().fold(0, |score, cube| {
        score + neigh(cube).iter().fold(0, |tmp, n| tmp + if cubes.contains(n) { 0 } else { 1 })
    });

    println!("Total: {}", total);

    let mut water = HashSet::new();
    let mut queue = vec![vec![0, 0, 0]];
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        for neigh in neigh(&current).iter().filter(|&n| !cubes.contains(n) && !water.contains(n) && n[0] >= -1 && n[0] <= 20 && n[1] >= -1 && n[1] <= 20 && n[2] >= -1 && n[2] <= 20) {
            queue.push(neigh.clone());
        }
        water.insert(current);
    }

    // Count the number of faces.
    let mut total = 0;
    for w in water.iter() {
        for n in neigh(w).iter() {
            if cubes.contains(n) {
                total += 1;
            }
        }
    }
    println!("Res 2: {}", total);
}

fn neigh(node: &[i32]) -> Vec<Vec<i32>> {
    let x = node[0];
    let y = node[1];
    let z = node[2];
    vec![
        vec![x, y, z + 1], vec![x, y, z - 1],
        vec![x, y + 1, z], vec![x, y - 1, z],
        vec![x + 1, y, z], vec![x - 1, y, z],
    ]
}