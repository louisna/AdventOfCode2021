use std::collections::{HashSet, HashMap};

fn main() {
    let content = include_str!("../../inputs/17.txt");

    let rocks = vec![
        vec![vec![true, true, true, true]],
        vec![vec![false, true, false], vec![true, true, true], vec![false, true, false]],
        vec![vec![false, false, true], vec![false, false, true], vec![true, true, true]],
        vec![vec![true], vec![true], vec![true], vec![true]],
        vec![vec![true, true], vec![true, true]],
    ];

    let mut rocks2 = rocks.iter().enumerate().cycle();

    let mut winds = content.chars().map(|i| match i {
        '>' => 1,
        '<' => -1,
        _ => panic!("Unknown"),
    }).enumerate().cycle();

    let nb_iters: i64 = 1000000000000;

    let mut nb_floor = 0;
    let mut floor: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..7 {
        floor.insert((0, i));
    }

    let mut map = HashMap::new();

    let mut look_for_cycle = true;
    let mut iter = 0;
    let mut final_add = None;

    while iter < nb_iters {
        iter += 1;
        let a = rocks2.next();
        let (rock_idx, rock) = a.as_ref().unwrap();
        let mut height = nb_floor + 3 + rock.len();
        let mut from_left = 2;
        loop {
            let (wind_idx, wind) = winds.next().unwrap();
            //let wind = 0;
            if can_move(rock, from_left, height as i32, wind, &floor) {
                from_left += wind;
            }

            //println!("Test drop");
            if can_drop(rock, from_left as usize, height, &floor) {
                height -= 1;
                //println!("Can drop to {}", height);
            } else {
                // Blocked here.

                // Add values of the rock.
                for yy in 0..rock.len() {
                    for xx in 0..rock[yy].len() {
                        if rock[yy][xx] {
                            floor.insert((height - yy, from_left as usize + xx));
                        }
                    }
                }

                let cycle_length = 1000;
                
                if look_for_cycle && height > cycle_length {
                    if map.contains_key(&(wind_idx, *rock_idx, from_left)) {
                        look_for_cycle = false;
                        let (nb_cycle_floor, nb_cycle_iter) = map.get(&(wind_idx, *rock_idx, from_left)).unwrap();
                        println!("Cycle found! {} {}", nb_cycle_floor, nb_cycle_iter);
                        println!("This cycle for {} {} {}", wind_idx, rock_idx, from_left);
                        println!("Currently: {}, cycle length={}", iter, iter - nb_cycle_iter);
                        let cycle_diff = iter - nb_cycle_iter;
                        let rocks_left = (nb_iters - iter) as f64;
                        let height_diff = nb_floor.max(height) - nb_cycle_floor;
                        println!("Le cycle loop: {}, actuellement: {}. Diff={}", nb_cycle_floor, nb_floor.max(height), height_diff);
                        println!("Cycle diff: {}, rocks let: {}", cycle_diff, rocks_left);
                        let batches = (rocks_left / cycle_diff as f64) -1.0;
                        println!("Batches: {}", batches);
                        let batches = batches.floor() as i64;
                        let to_add = height_diff as i64 * batches;
                        final_add = Some(to_add as usize);
                        println!("New value: {}", nb_floor.max(height));
                        println!("Iter={} + {}. To add={}", iter, batches * cycle_diff, to_add);
                        iter += batches * cycle_diff;
                        println!("Batches: {}, to add: {:?}", batches, final_add);
                        println!("New iter={}", iter);
    
                        // I have to push the new items on top.
                    } else if look_for_cycle {
                        // println!("J'insere {} {} {}", wind_idx, rock_idx, from_left);
                        map.insert((wind_idx, *rock_idx, from_left), (nb_floor.max(height), iter));
                    }
                }



                nb_floor = height.max(nb_floor);
                // println!("Current nb={}", nb_floor);
                break;
            }
        }
    }

    let mut a: Vec<_> = floor.iter().map(|k| k).collect();
    a.sort();

    //println!("{:?}", a);
    println!("Res 1: {}", nb_floor + final_add.unwrap_or(0));


}

fn can_move(rock: &[Vec<bool>], cur_x: i32, cur_y: i32, moving: i32, presence: &HashSet<(usize, usize)>) -> bool {
    let new_x = cur_x + moving;
    if new_x < 0 || new_x + rock[0].len() as i32 > 7 {
        return false;
    }

    for i in 0..rock.len() {
        for j in 0..rock[i].len() {
            if rock[i][j] {
                if presence.contains(&(cur_y as usize - i, new_x as usize + j)) {
                    return false;
                }
            }
        }
    }
    true

}

fn can_drop(rock: &[Vec<bool>], cur_x: usize, cur_y: usize, floor: &HashSet<(usize, usize)>) -> bool {
    // No data for this height, so it means that it is free.
    // if cur_floor < cur_y + 1 - rock.len() {
    //     return true;
    // }

    // Check every floor.
    for i in (0..rock.len()).rev() {
        let y = cur_y - i - 1;
        for j in 0..rock[0].len() {
            let x = cur_x + j;
            if rock[i][j] && floor.contains(&(y, x)) {
                //println!("Cannot move from {} {} because {} {} exists", cur_x, cur_y, y, x);
                return false;
            }
        }
    }
    return true;




}