use std::collections::{HashMap, HashSet};

fn main() {
    let content = include_str!("../../inputs/23.txt");
    let mut elves: HashSet<(i32, i32)> = HashSet::from_iter(
        content
            .split('\n')
            .enumerate()
            .map(|(i, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .map(move |(j, &c)| match c {
                        b'.' => None,
                        b'#' => Some((i as i32, j as i32)),
                        _ => panic!("Unknown char"),
                    })
            })
            .flatten()
            .filter_map(|f| f),
    );

    println!("Init: {:?}", elves);

    let moves = [
        ((-1, 0), [(-1, 0), (-1, -1), (-1, 1)]),
        ((1, 0), [(1, 0), (1, -1), (1, 1)]),
        ((0, -1), [(-1, -1), (0, -1), (1, -1)]),
        ((0, 1), [(1, 1), (0, 1), (-1, 1)])
    ];

    nice_print(&elves);

    let mut i_move = 0;
    loop {
        if i_move == 10 {
            let (min_x, max_x, min_y, max_y) = find_rectangle(&elves);
            let res = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32;
            println!("Res after 10: {}", res);
        }
        // First half. Elves choose the movement.
        let mut both_want = HashSet::new(); // Will be used to check for duplicates.
        let mut next_elves = HashSet::new();
        let mut new_to_old: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        'elves_loop: for &elve_pos in elves.iter() {
            for next_move in 0..4 {
                let idx_move = (i_move + next_move) % 4;
                if let Some(next_pos) = check_move(&elves, elve_pos, &moves[idx_move]) {
                    // Can move. Check if another elve tries to go to the same place.
                    if next_elves.contains(&next_pos) {
                        // Arf... Another elve want to go to the same place.
                        both_want.insert(next_pos);
                        // The elve will stay on its place... but it is already, so we can add it already in the next_elves.
                        next_elves.insert(elve_pos);
                    } else {
                        next_elves.insert(next_pos);
                        new_to_old.insert(next_pos, elve_pos);
                    }
                    continue 'elves_loop; // Skip other movements.
                }
            }
            // No move found... Stays...
            next_elves.insert(elve_pos);
        }
        // The elves can move now... Only if the position is not chosen by multiple elves.
        for dup_pos in both_want.iter() {
            next_elves.remove(dup_pos);
            
            // But also... this elves stays where it is.
            next_elves.insert(*new_to_old.get(dup_pos).unwrap());
        }

        // No elve moved.
        if elves == next_elves {
            println!("No move at the end of round {}", i_move + 1);
            return;
        }
        
        elves = next_elves;

        // At the end... print to be sure it works.
        // nice_print(&elves);

        i_move += 1;
    }


}

fn check_move(current_map: &HashSet<(i32, i32)>, pos: (i32, i32), movement: &((i32, i32), [(i32, i32); 3])) -> Option<(i32, i32)> {
    // First check if the elve can stay where it is currently.
    let mut can_stay = true;
    'extern_loop: for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            if current_map.contains(&(pos.0 + i, pos.1 + j)) {
                can_stay = false;
                break 'extern_loop;
            }
        }
    }
    if can_stay {
        return Some(pos); // Stay where it is
    }
    
    let new_pos: (i32, i32) = (pos.0 + movement.0.0, pos.1 + movement.0.1);

    // Check that nobody is on the new attempted position.
    for &(i_check, j_check) in movement.1.iter() {
        let check_pos = (pos.0 + i_check, pos.1 + j_check);
        if current_map.contains(&check_pos) {
            return None;
        }
    }

    Some(new_pos)
}

fn nice_print(elves: &HashSet<(i32, i32)>) {
    let (min_x, max_x, min_y, max_y) = find_rectangle(elves);
    for i in min_x..max_x + 1 {
        for j in min_y..max_y + 1 {
            if elves.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn find_rectangle(elves: &HashSet<(i32, i32)>) -> (i32, i32, i32, i32) {
    let min_x = elves.iter().map(|&(i, _)| i).min().unwrap();
    let max_x = elves.iter().map(|&(i, _)| i).max().unwrap();
    let min_y = elves.iter().map(|&(_, j)| j).min().unwrap();
    let max_y = elves.iter().map(|&(_, j)| j).max().unwrap();
    (min_x, max_x, min_y, max_y)
}