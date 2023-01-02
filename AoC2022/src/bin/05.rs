use std::collections::LinkedList;

fn main() {
    let content = include_str!("../../inputs/05.txt");
    let lines: Vec<&str> = content.split("\n").collect();
    let nb = (((lines[0].len() - 2) as f32) / 4.0).ceil() as usize;
    let nb_descr = lines.iter().position(|l| l.is_empty()).unwrap() - 1;
    
    let mut crates: Vec<LinkedList<char>> = vec![LinkedList::new(); nb];
    
    for line in &lines[..nb_descr] {
        let mut idx = 0;
        for c in line.chars().skip(1).step_by(4) {
            if c != ' ' {
                crates[idx].push_back(c);
            }
            idx += 1;
        }
    }

    // Now the instructions.
    // for instruction in lines[nb_descr + 2..].iter() {
    //     let s: Vec<&str> = instruction.split(" ").collect();
    //     let nb_move = s[1].parse::<u8>().unwrap() as usize;
    //     let from = s[3].parse::<u8>().unwrap() as usize - 1;
    //     let to = s[5].parse::<u8>().unwrap() as usize - 1;
    //     println!("Move {} from {} to {}", nb_move, from, to);

    //     for _ in 0..nb_move {
    //         let c = crates[from].pop_front().unwrap();
    //         crates[to].push_front(c);
    //     }
    // }

    for instruction in lines[nb_descr + 2..].iter() {
        let s: Vec<&str> = instruction.split(" ").collect();
        let nb_move = s[1].parse::<u8>().unwrap() as usize;
        let from = s[3].parse::<u8>().unwrap() as usize - 1;
        let to = s[5].parse::<u8>().unwrap() as usize - 1;
        println!("Move {} from {} to {}", nb_move, from, to);

        let mut v = LinkedList::new();

        for _ in 0..nb_move {
            let c = crates[from].pop_front().unwrap();
            v.push_front(c);
        }

        for _ in 0..nb_move {
            let c = v.pop_front().unwrap();
            crates[to].push_front(c);
        }
    }

    // Result.
    let res: Vec<&char> = crates.iter().map(|l| l.front().unwrap()).collect();
    let res: String = res.into_iter().collect();
    println!("Res 1: {}", res);
}
