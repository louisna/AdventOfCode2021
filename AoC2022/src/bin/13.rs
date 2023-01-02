use serde_json;
use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
enum List {
    Int(i8),
    Vec(Vec<List>),
}

fn main() {
    let content = include_str!("../../inputs/13.txt");
    let pairs_lines: Vec<_> = content.split("\n\n").collect();
    println!("Pairs is {:?}", pairs_lines);

    let pairs: Vec<_> = pairs_lines.iter().map(|pair_line| {
        let mut split = pair_line.split('\n');
        let first = split.next().unwrap();
        let first: List = serde_json::from_str(first).unwrap();
        let second = split.next().unwrap();
        let second: List = serde_json::from_str(second).unwrap();
        (first, second)
    }).collect();

    let results: usize = pairs.iter().map(|(a, b)| cmp2(a, b)).enumerate().filter(|&(_, v)| v == Ordering::Less).map(|(idx, _)| idx + 1).sum();
    println!("Res 1: {}", results);

    let divider_2: List = serde_json::from_str("[[2]]").unwrap();
    let divider_6: List = serde_json::from_str("[[6]]").unwrap();

    let mut all_data = Vec::with_capacity(pairs.len() * 2);
    pairs.iter().for_each(|(a, b)| {
        all_data.push(a);
        all_data.push(b);
    });

    all_data.push(&divider_2);
    all_data.push(&divider_6);

    all_data.sort_by(|a, b| cmp2(a, b));

    let idx_2 = all_data.iter().position(|&p| p == &divider_2).unwrap() + 1;
    let idx_6 = all_data.iter().position(|&p| p == &divider_6).unwrap() + 1;

    println!("Res 2: {}", idx_2 * idx_6);

}

fn cmp2(a: &List, b: &List) -> Ordering {
    match (a, b) {
        (List::Int(aa), List::Int(bb)) => aa.cmp(bb),
        (List::Vec(va), List::Vec(vb)) => {
            for i in 0..va.len().min(vb.len()) {
                let c = cmp2(&va[i], &vb[i]);
                if c == Ordering::Equal {
                    continue;
                }
                return c;
            }
            if va.len() < vb.len() {
                return Ordering::Less;
            } else if va.len() == vb.len() {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        },
        (List::Int(aa), List::Vec(_)) => cmp2(&List::Vec(vec![List::Int(*aa)]), b),
        (List::Vec(_), List::Int(bb)) => cmp2(a, &List::Vec(vec![List::Int(*bb)])),
    }
}