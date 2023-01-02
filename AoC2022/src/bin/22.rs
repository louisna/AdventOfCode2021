use std::collections::HashMap;

const RIGHT: u8 = 0;
const DOWN: u8 = 1;
const LEFT: u8 = 2;
const UP: u8 = 3;

fn main() {
    let content = include_str!("../../inputs/22.txt");

    let split: Vec<_> = content.split("\n\n").collect();
    let map_str = split[0];
    let moves: Vec<_> = split[1].chars().collect();

    // Complete the map
    let map: HashMap<(i32, i32), bool> = HashMap::from_iter(map_str.split('\n').enumerate().map(|(y, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .map(move |(x, char)| match char {
                b'.' => Some(((x as i32, y as i32), true)),
                b'#' => Some(((x as i32, y as i32), false)),
                _ => None,
            }).flatten()
    }).flatten());

    let mut changes = HashMap::new();
    get_change_x(&mut changes);
    get_change_y(&mut changes);


    // let mut position = *map.iter().filter(|&(&(_, y), b)| y == 0 && *b).min().unwrap().0;
    // println!("Init position: {:?}", position);
    // let mut direction = RIGHT; 

    // let mut i = 0;
    // 'extern_loop: loop {
    //     if moves.len() <= i {
    //         break;
    //     }
    //     let c = moves[i];
    //     i += 1;
    //     println!("Position is :{:?}, direction: {}", (position.1 + 1, position.0 + 1), direction);
    //     match c {
    //         'L' => direction = (4 + direction - 1) % 4,
    //         'R' => direction = (direction + 1) % 4,
    //         digit => {
    //             let mut nb = digit.to_digit(10).unwrap();
    //             while i < moves.len() {
    //                 let d = moves[i];
    //                 match d.to_digit(10) {
    //                     Some(converted) => nb = nb * 10 + converted,
    //                     None => break,
    //                 }
    //                 i += 1;
    //             }
    //             // println!("Number of moves: {}", nb);
    //             for _ in 0..nb {
    //                 (position, direction) = match can_move(position, direction, &map, &changes) {
    //                     Some(v) => v,
    //                     None => continue 'extern_loop,
    //                 };
    //             }
    //         },
    //     }
    // }

    // println!("Position: {:?}", position);

    // let res = (position.1 + 1) * 1000 + (position.0 + 1) * 4 + direction as i32;
    // println!("Res: {}", res);

    // Part 2.
    let cube = map_to_cube(&map);
    let mut cube_changes = HashMap::new();
    cube_moves(&mut cube_changes);
    let mut position = (1, 0, 0);
    let mut direction = RIGHT;
    let mut i = 0;
    'extern_loop: loop {
        if moves.len() <= i {
            break;
        }
        let c = moves[i];
        i += 1;
        println!("Position is :{:?}, direction: {} (initial is {:?})", to_values(position.0, position.1, position.2), direction, position);
        match c {
            'L' => direction = (4 + direction - 1) % 4,
            'R' => direction = (direction + 1) % 4,
            digit => {
                let mut nb = digit.to_digit(10).unwrap();
                while i < moves.len() {
                    let d = moves[i];
                    match d.to_digit(10) {
                        Some(converted) => nb = nb * 10 + converted,
                        None => break,
                    }
                    i += 1;
                }
                // println!("Number of moves: {}", nb);
                for _ in 0..nb {
                    (position, direction) = match can_move_cube(position, direction, &cube, &cube_changes) {
                        Some(v) => v,
                        None => continue 'extern_loop,
                    };
                }
            },
        }
    }

    println!("Final position is: {:?}", position);
    let res = to_values(position.0, position.1, position.2);
    let res = 1000 * res.0 + 4 * res.1 + direction as i32;
    println!("Res: {}", res);

}

fn can_move_cube(pos: (i32, i32, i32), direction: u8, cube: &HashMap<(i32, i32, i32), bool>, moves: &HashMap<(i32, i32, i32, u8), (i32, i32, i32, u8)>) -> Option<((i32, i32, i32), u8)> {
    let dir_pos = match direction {
        RIGHT => (0, 1),
        LEFT => (0, -1),
        UP => (-1, 0),
        DOWN => (1, 0),
        _ => panic!("Unknown move"),
    };
    let new_pos = (pos.0, pos.1 + dir_pos.0, pos.2 + dir_pos.1);
    //println!("New pos is {:?}", new_pos);

    //println!("For pos: {:?}, direction {}", pos, direction);
    match cube.get(&new_pos) {
        Some(false) => None,
        Some(true) => Some((new_pos, direction)),
        None => {
            let moved = moves.get(&(pos.0, pos.1, pos.2, direction)).unwrap();
            //println!("Moved value is {:?}", moved);
            if *cube.get(&(moved.0, moved.1, moved.2)).unwrap() {
                Some(((moved.0, moved.1, moved.2), moved.3))
            } else {
                None
            }
        }
    } 
}

fn _can_move(pos: (i32, i32), direction: u8, map: &HashMap<(i32, i32), bool>, changes: &HashMap<(i32, i32, u8), (i32, i32)>) -> Option<((i32, i32), u8)> {
    let dir_pos = match direction {
        RIGHT => (1, 0),
        LEFT => (-1, 0),
        UP => (0, -1),
        DOWN => (0, 1),
        _ => panic!("Unknown move"),
    };
    let new_pos = (pos.0 + dir_pos.0, pos.1 + dir_pos.1);
    // println!("New pos: {:?}", new_pos);
    match map.get(&new_pos) {
        Some(false) => None,
        Some(true) => Some((new_pos, direction)),
        None => {
            // println!("Cherche moved pour {:?} {} car {:?} n'est pas bon", pos, direction, new_pos);
            let moved = changes.get(&(pos.0, pos.1, direction)).unwrap();
            if *map.get(moved).unwrap() {
                Some((*moved, direction))
            } else {
                None
            }
        }
    }
}

fn get_change_x(map: &mut HashMap<(i32, i32, u8), (i32, i32)>) {
    for i in 0..50 {
        map.insert((50, i, LEFT), (149, i));
        map.insert((149, i, RIGHT), (50, i));
    }
    for i in 50..100 {
        map.insert((50, i, LEFT), (99, i));
        map.insert((99, i, RIGHT), (50, i));
    }
    for i in 100..150 {
        map.insert((0, i, LEFT), (99, i));
        map.insert((99, i, RIGHT), (0, i));
    }
    for i in 150..200 {
        map.insert((0, i, LEFT), (49, i));
        map.insert((49, i, RIGHT), (0, i));
    }
}

fn get_change_y(map: &mut HashMap<(i32, i32, u8), (i32, i32)>) {
    for i in 0..50 {
        map.insert((i, 100, UP), (i, 199));
        map.insert((i, 199, DOWN), (i, 100));
    }
    for i in 50..100 {
        map.insert((i, 0, UP), (i, 149));
        map.insert((i, 149, DOWN), (i, 0));
    }
    for i in 100..150 {
        map.insert((i, 0, UP), (i, 49));
        map.insert((i, 49, DOWN), (i, 0));
    }
}

fn map_to_cube(map: &HashMap<(i32, i32), bool>) -> HashMap<(i32, i32, i32), bool> {
    let mut cube = HashMap::new();

    let faces = [(0, 0, 100), (1, 0, 50), (2, 50, 50), (3, 100, 50), (4, 100, 0), (5, 150, 0)];
    for &(face, xd, yd) in faces.iter() {
        for i in 0..50 {
            for j in 0..50 {
                if face == 4 && i == 49 {
                    println!("Face {}: {:?} -> {:?}", face, (yd + j, xd + i), (i, j));
                }
                cube.insert((face, i, j), *map.get(&(yd + j, xd + i)).unwrap());
            }
        }
    }

    cube
}

fn cube_moves(map: &mut HashMap<(i32, i32, i32, u8), (i32, i32, i32, u8)>) {
    // Face 1 to 6.
    for i in 0..50 {
        map.insert((0, 0, i, UP), (5, 49, i, UP));
    }
    // Face 1 to 3.
    for i in 0..50 {
        map.insert((0, 49, i, DOWN), (2, i, 49, LEFT));
    }
    // Face 1 to 2.
    for i in 0..50 {
        map.insert((0, i, 0, LEFT), (1, i, 49, LEFT));
    }
    // Face 1 to 4.
    for i in 0..50 {
        map.insert((0, i, 49, RIGHT), (3, (49 - i), 49, LEFT));
    }

    // Face 2 to 6.
    for i in 0..50 {
        map.insert((1, 0, i, UP), (5, i, 0, RIGHT));
    }
    // Face 2 to 3.
    for i in 0..50 {
        map.insert((1, 49, i, DOWN), (2, 0, i, DOWN));
    }
    // Face 2 to 5.
    for i in 0..50 {
        map.insert((1, i, 0, LEFT), (4, (49 - i), 0, RIGHT));
    }
    // Face 2 to 1.
    for i in 0..50 {
        map.insert((1, i, 49, RIGHT), (0, i, 0, RIGHT));
    }

    // Face 3 to 2.
    for i in 0..50 {
        map.insert((2, 0, i, UP), (1, 49, i, UP));
    }
    // Face 3 to 4.
    for i in 0..50 {
        map.insert((2, 49, i, DOWN), (3, 0, i, DOWN));
    }
    // Face 3 to 5.
    for i in 0..50 {
        map.insert((2, i, 0, LEFT), (4, 0, i, DOWN));
    }
    // Face 3 to 1.
    for i in 0..50 {
        map.insert((2, i, 49, RIGHT), (0, 49, i, UP));
    }

    // Face 4 to 3.
    for i in 0..50 {
        map.insert((3, 0, i, UP), (2, 49, i, UP));
    }
    // Face 4 to 6.
    for i in 0..50 {
        map.insert((3, 49, i, DOWN), (5, i, 49, LEFT));
    }
    // Face 4 to 5.
    for i in 0..50 {
        map.insert((3, i, 0, LEFT), (4, i, 49, LEFT));
    }
    // Face 4 to 1.
    for i in 0..50 {
        map.insert((3, i, 49, RIGHT), (0, (49 - i), 49, LEFT));
    }

    // Face 5 to 3.
    for i in 0..50 {
        map.insert((4, 0, i, UP), (2, i, 0, RIGHT));
    }
    // Face 5 to 6.
    for i in 0..50 {
        map.insert((4, 49, i, DOWN), (5, 0, i, DOWN));
    }
    // Face 5 to 2.
    for i in 0..50 {
        map.insert((4, i, 0, LEFT), (1, (49 - i), 0, RIGHT));
    }
    // Face 5 to 4.
    for i in 0..50 {
        map.insert((4, i, 49, RIGHT), (3, i, 0, RIGHT));
    }

    // Face 6 to 5.
    for i in 0..50 {
        map.insert((5, 0, i, UP), (4, 49, i, UP));
    }
    // Face 6 to 1.
    for i in 0..50 {
        map.insert((5, 49, i, DOWN), (0, 0, i, DOWN));
    }
    // Face 6 to 2.
    for i in 0..50 {
        map.insert((5, i, 0, LEFT), (1, 0, i, DOWN));
    }
    // Face 6 to 4.
    for i in 0..50 {
        map.insert((5, i, 49, RIGHT), (3, 49, i, UP));
    }
}

fn to_values(face: i32, x: i32, y: i32) -> (i32, i32) {
    let map = [(0, (0, 100)), (1, (0, 50)), (2, (50, 50)), (3, (100, 50)), (4, (100, 0)), (5, (150, 0))];
    let (_, v) = map[face as usize];
    (x + v.0 + 1, y + v.1 + 1)
}