use std::collections::HashMap;

fn main() {
    let content = include_str!("../../inputs/20.txt");
    let mut encrypted: Vec<(usize, i64)> = content.split('\n').enumerate().map(|(idx, i)| (idx, i.parse::<i64>().unwrap() * 811589153)).collect();
    // let mut encrypted: Vec<(usize, i64)> = encrypted.iter().map(|&(idx, value)| (idx, value % encrypted.len() as i64)).collect();
    println!("ORIGINAL: {:?}", encrypted);
    let order = encrypted.clone();

    let mut position: HashMap<(usize, i64), usize> = HashMap::from_iter(order.iter().enumerate().map(|(idx, (_, value))| ((idx, *value), idx)));

    for i in 0..10 {
        println!("ITER: {}", i);
        for (idx, &(value_idx, value)) in order.iter().enumerate() {
            if value == 0 {
                continue;
            }
            let moving = value / value.abs();
            // let nb_total_moves = value.abs() / encrypted.len() as i64;
            // let copied = encrypted.clone();
            // for idx in 0..copied.len() {
            //     encrypted[(idx as i64 + nb_total_moves * moving) as usize % copied.len()] = copied[idx];
            // }
            for _ in 0..(value.abs() % (encrypted.len() - 1) as i64) as usize {
                let current_pos = *position.get(&(idx, value)).unwrap();
                let moved_idx = (encrypted.len() as i64 + current_pos as i64 + moving) as usize % encrypted.len();
                encrypted[current_pos] = encrypted[moved_idx];
                encrypted[moved_idx] = (value_idx, value);
    
                // Update position.
                position.insert((idx, value), moved_idx);
                position.insert(encrypted[current_pos], current_pos);
            }
        }
    }

    // Get position of 0.
    let pos_0 = encrypted.iter().map(|(_, v)| v).position(|&v| v == 0).unwrap();
    let res = encrypted[(pos_0 + 1000) % encrypted.len()].1 + encrypted[(pos_0 + 2000) % encrypted.len()].1 + encrypted[(pos_0 + 3000) % encrypted.len()].1;
    println!("ENCRYPTED: {:?}", encrypted);
    // println!("Res: {}", res);
    println!("Res: {}", res);
    
}