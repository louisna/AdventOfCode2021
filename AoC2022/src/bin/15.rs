use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    closest: i32, // Manhattan distance of closest to the point.
    is_beacon: bool,
}

fn main() {
    let content = include_str!("../../inputs/15.txt");

    let points: Vec<_> = content
        .split('\n')
        .flat_map(|line| {
            let line: Vec<&str> = line.split(' ').collect();
            let x = line[2][2..line[2].len() - 1].parse::<i32>().unwrap();
            let y = line[3][2..line[3].len() - 1].parse::<i32>().unwrap();

            let cx = line[8][2..line[8].len() - 1].parse::<i32>().unwrap();
            let cy = line[9][2..].parse::<i32>().unwrap();

            let sensor = Point {
                x,
                y,
                closest: mnd((x, y), (cx, cy)),
                is_beacon: false,
            };
            let beacon = Point {
                x: cx,
                y: cy,
                closest: mnd((x, y), (cx, cy)),
                is_beacon: true,
            };

            vec![sensor, beacon]
        })
        .collect();

    let beacons: Vec<_> = points.iter().filter(|b| b.is_beacon).collect();
    let sensors: Vec<_> = points.iter().filter(|s| !s.is_beacon).collect();

    let mut beacons_position = HashSet::new();
    for beacon in &beacons {
        beacons_position.insert((beacon.x, beacon.y));
    }

    let min_x = sensors.iter().map(|p| p.x - p.closest).min().unwrap();
    let max_x = sensors.iter().map(|p| p.x + p.closest).max().unwrap();

    // let y = 2000000;
    // let mut total = 0;
    // 'ext: for i in min_x..max_x {
    //     if beacons_position.contains(&(i, y)) {
    //         continue;
    //     }
    //     for point in sensors.iter() {
    //         if mnd((i, y), (point.x, point.y)) <= point.closest {
    //             total += 1;
    //             // println!("Val: {},{}", i, y);
    //             continue 'ext;
    //         }
    //     }
    // }
    // println!("Res 1: {}", total);

    let lim = 4000000;
    for x in 0..lim {
        let mut y = 0;
        if x % 100_000 == 0 {
            println!("Increase x: {}", x);
        }
        'loop_y: while y < lim {
            for point in sensors.iter() {
                let dist = mnd((x, y), (point.x, point.y));
                if dist == point.closest {
                    y += 1;
                    continue 'loop_y;
                } else if dist < point.closest {
                    // Not candidate.
                    let how_much = point.closest - dist;
                    // println!("Skip some: {} -> {}", y, y + how_much);
                    y += how_much;
                    continue 'loop_y;
                }
            }
            println!("Coordinates: {},{}. Res={}", x, y, (x as i64) * 4000000 + (y as i64));
            return;
        }
    }
}

fn mnd(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
