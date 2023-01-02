#[macro_use]
extern crate log;

use std::collections::{HashSet, VecDeque};

const VERTICAL: bool = true;
const HORIZONTAL: bool = false;

const TO_GOAL: u8 = 0;
const TO_INIT: u8 = 1;
const TO_GOAL_FINAL: u8 = 2;

fn main() {
    env_logger::init();
    let content = include_str!("../../inputs/24.txt");
    
    let mut map = HashSet::new();
    let lines: Vec<_> = content.split('\n').collect();
    
    let nb_rows = lines.len() as i32 - 2;
    let nb_cols = lines[0].len() as i32 - 2;
    let starting_pos = (0, -1, 0);

    for (i, line) in lines[1..lines.len() - 1].iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();
        for (j, c) in chars[1..chars.len() - 1].iter().enumerate() {
            let i = i as i32;
            let j = j as i32;
            match c {
                '^' => {
                    let nb_rows_iter = if j == nb_cols - 1 {
                        debug!("Panic");
                        nb_rows + 1
                    } else {
                        nb_rows
                    };
                    for nb in 0..nb_rows_iter {
                        map.insert((nb, (nb_rows + i - nb) % nb_rows_iter, j, VERTICAL));
                    }                    
                },
                'v' => {
                    let nb_rows_iter = if j == nb_cols - 1 {
                        debug!("Panic 2");
                        nb_rows + 1
                    } else {
                        nb_rows
                    };
                    for nb in 0..nb_rows_iter {
                        map.insert((nb, (i + nb) % nb_rows_iter, j, VERTICAL));
                    }
                },
                '>' => {
                    for nb in 0..nb_cols {
                        map.insert((nb, i, (j + nb) % nb_cols, HORIZONTAL));
                    }
                },
                '<' => {
                    for nb in 0..nb_cols {
                        map.insert((nb, i, (nb_cols + j - nb) % nb_cols, HORIZONTAL));
                    }
                },
                '.' => (), // Do nothing for empty spaces.
                _ => panic!("Unknown char"),
            }
        }
    }

    debug!("This is the map: {:?}", map);

    let mut queue = VecDeque::new();
    queue.push_back(starting_pos);
    let mut visited = HashSet::new();
    let mut direction = TO_GOAL;

    while !queue.is_empty() {
        let (cost, i, j) = queue.pop_front().unwrap();
        if visited.contains(&(cost, i, j)) {
            continue;
        }
        visited.insert((cost, i, j));

        // Check if it is goal.
        if i == nb_rows && j == nb_cols - 1 {
            if direction == TO_GOAL {
                println!("Res 1: {}", cost);
                println!("Going back...");
                queue = VecDeque::new();
                queue.push_back((cost, i, j));
                direction = TO_INIT;
                visited.remove(&(cost, i, j));
                continue;
            } else if direction == TO_GOAL_FINAL {
                println!("Final result: {}", cost);
                return;
            }
        }

        // Check if initial.
        if i == -1 && j == 0 && direction == TO_INIT {
            println!("Reaches back the init: {}. Going to final again...", cost);
            direction = TO_GOAL_FINAL;
            queue = VecDeque::new();
            queue.push_back((cost, i, j));
            visited.remove(&(cost, i, j));
            continue;
        }

        // Is next move a goal and we want to go to the goal?
        if (direction == TO_GOAL || direction == TO_GOAL_FINAL) && i + 1 == nb_rows && j == nb_cols - 1 && can_move(&map, i + 1, j, cost + 1, nb_rows, nb_cols) {
            queue.push_back((cost + 1, nb_rows, j));
            continue; // Do no check other moves if we can reach the goal.
            // TODO: maybe there is wind on the goal?
        }

        // Is next move the init and we have to come back?
        if direction == TO_INIT && i - 1 == -1 && j == 0 {
            queue.push_back((cost + 1, -1, 0));
            continue;
        }

        // Can move down?
        if i < nb_rows - 1 && can_move(&map, i + 1, j, cost + 1, nb_rows, nb_cols) {
            queue.push_back((cost + 1, i + 1, j));
        }

        // Can move right?
        if j < nb_cols - 1 && i >= 0 && i < nb_rows && can_move(&map, i, j + 1, cost + 1, nb_rows, nb_cols) {
            queue.push_back((cost + 1, i, j + 1));
        }

        // Can stay?
        if can_move(&map, i, j, cost + 1, nb_rows, nb_cols) {
            queue.push_back((cost + 1, i, j));
        }

        // Can move up?
        if i > 0 && can_move(&map, i - 1, j, cost + 1, nb_rows, nb_cols) {
            queue.push_back((cost + 1, i - 1, j));
        }

        // Can move left?
        if j > 0 && i >= 0 && i < nb_rows && can_move(&map, i, j - 1, cost + 1, nb_rows, nb_cols) {
            queue.push_back((cost + 1, i, j - 1));
        }
    }
}

fn can_move(map: &HashSet<(i32, i32, i32, bool)>, i: i32, j: i32, time: i32, nb_rows: i32, nb_cols: i32) -> bool {
    let ca = map.contains(&(time % nb_rows, i, j, VERTICAL));
    let cb = map.contains(&(time % nb_cols, i, j, HORIZONTAL));
    // println!("At time {}, position {} {}, contain vertical: {}, contain horizontal: {}", time, i, j, ca, cb);
    !(ca || cb)
}