use std::collections::HashSet;

fn main() {
    let content = include_str!("../../inputs/09.txt");

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    
    let mut rope = [(0i64, 0i64); 10];
    
        visited.insert(rope[9]);

    for line in content.split("\n") {
        let mut tab = line.split(" ");
        let direction = tab.next().unwrap();
        let nb: usize = tab.next().unwrap().parse().unwrap();
        'l: for _ in 0..nb {
            let h = rope[0]; // Old head.
            rope[0] = match direction {
                "R" => (h.0, h.1 + 1),
                "L" => (h.0, h.1 - 1),
                "U" => (h.0 + 1, h.1),
                "D" => (h.0 - 1, h.1),
                _ => panic!("eeeeee"),
            };
            for i in 1..10 {
                let diff = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if diff.0.abs() <= 1 && diff.1.abs() <= 1 {
                    continue 'l;
                }
                if diff.0.abs() > 1 {
                    rope[i].0 += diff.0 / diff.0.abs();
                    if diff.1 != 0 {
                        rope[i].1 += diff.1 / diff.1.abs();
                    }
                }
                else if diff.1.abs() > 1 {
                    rope[i].1 += diff.1 / diff.1.abs();
                    if diff.0 != 0 {
                        rope[i].0 += diff.0 / diff.0.abs();
                    }
                }
            }
            
            visited.insert(rope[9]);
        }
        println!("Queue is {:?}", rope);
    }
    
    println!("Res 1: {}", visited.len());
}

