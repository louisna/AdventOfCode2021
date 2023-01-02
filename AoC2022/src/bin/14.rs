use std::collections::HashSet;

const PART_2: bool = true;

fn main() {
    let content = include_str!("../../inputs/14.txt");
    let lines: Vec<_> = content.split('\n').collect();
    let limits: Vec<Vec<(usize, usize)>> = lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|val| {
                    let mut tab = val.split(",");
                    let x = tab.next().unwrap().parse::<usize>().unwrap();
                    let y = tab.next().unwrap().parse::<usize>().unwrap();
                    (x, y)
                })
                .collect()
        })
        .collect();

    // Compute the limits of the grid.
    let min_x = limits
        .iter()
        .map(|line| line.iter().map(|(x, _)| *x).min().unwrap())
        .min()
        .unwrap();
    let max_x = limits
        .iter()
        .map(|line| line.iter().map(|(x, _)| *x).max().unwrap())
        .max()
        .unwrap();
    let max_y = limits
        .iter()
        .map(|line| line.iter().map(|(_, y)| *y).max().unwrap())
        .max()
        .unwrap();

    // Fill the rocks in the map.
    let mut map = HashSet::new();
    for limit in limits.iter() {
        for line in limit.windows(2) {
            let from = line[0];
            let to = line[1];
            let delta = (
                from.0.max(to.0) - from.0.min(to.0),
                from.1.max(to.1) - from.1.min(to.1),
            );
            if delta.0 > 0 {
                // The line is on x-axis.
                for i in 0..delta.0 + 1 {
                    map.insert((from.0.min(to.0) + i, from.1));
                }
            } else {
                // the line is on y-axis.
                for i in 0..delta.1 + 1 {
                    map.insert((from.0, from.1.min(to.1) + i));
                }
            }
        }
    }

    // Simulate sand.
    let mut nb_sand = 0;
    'outer: loop {
        let mut sand = (500, 0);
        'inner: loop {
            if PART_2 {
                // Is the sand on the floor?
                if sand.1 == max_y + 1 {
                    map.insert(sand);
                    break 'inner;
                }
            } else {
                // Is the sand out of the box?
                if sand.0 < min_x || sand.0 > max_x || sand.1 > max_y {
                    break 'outer;
                }
            }

            if !map.contains(&(sand.0, sand.1 + 1)) {
                // Sand can go down.
                sand = (sand.0, sand.1 + 1);
            } else if !map.contains(&(sand.0 - 1, sand.1 + 1)) {
                // Sand can go down left.
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !map.contains(&(sand.0 + 1, sand.1 + 1)) {
                // Sand can go down right.
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                // Sand cannot go anywhere. Add sand position.
                map.insert(sand);
                break 'inner;
            }
        }
        nb_sand += 1;

        if sand == (500, 0) {
            // Sand cannot fit.
            break 'outer;
        }
    }

    println!("Res 2: {}", nb_sand);
}
