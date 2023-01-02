use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Node {
    op: Option<fn(f64, f64) -> f64>,
    waits_for: i8,
    waiting_list: Vec<(String, u8)>,
    value: Option<f64>,
}

fn main() {
    let content = include_str!("../../inputs/21.txt");
    let nodes: HashMap<&str, RefCell<Node>> = HashMap::from_iter(content.split('\n').map(|line| {
        let mut tab = line.split(':');
        (tab.next().unwrap(), RefCell::new(Node { op: None, waits_for: 0, waiting_list: Vec::new(), value: None }))
    }));

    let mut queue = VecDeque::new();

    content.split('\n').for_each(|line| {
        let tab: Vec<_> = line.split(" ").collect();
        let first = &tab[0][..tab[0].len() - 1];
        let mut first_node = nodes.get(first).unwrap().borrow_mut();
        if tab.len() == 2 {
            first_node.value = Some(tab[1].parse().unwrap());
            queue.push_back(first.to_string());
        } else {
            let second = &tab[1];
            let third = &tab[3];
            first_node.waits_for = 2;
            first_node.op = Some(match &tab[2] {
                &"+" => |a, b| a + b,
                &"-" => |a, b| a - b,
                &"*" => |a, b| a * b,
                &"/" => |a, b| a / b,
                _ => panic!("Unknown op"),
            });

            let mut second_node = nodes.get(second).unwrap().borrow_mut();
            second_node.waiting_list.push((first.to_string(), 0));

            let mut third_node = nodes.get(third).unwrap().borrow_mut();
            third_node.waiting_list.push((first.to_string(), 1));

        }
    });

    let res = iter(nodes.clone(), queue.clone());
    println!("Res: {}", res);

    // Part 2.
    nodes.get("root").unwrap().borrow_mut().op = Some(|a, b| a - b);
    let mut lower_bound: f64 = -1_000_000_000_000_000_000_000.0;
    let mut upper_bound: f64 = 1_000_000_000_000_000.0;
    let mut nb_iter = 0;
    loop {
        let mid = (lower_bound + upper_bound) / 2.0;
        nodes.get("humn").unwrap().borrow_mut().value = Some(mid);
        let res = iter(nodes.clone(), queue.clone());
        println!("Value of mid: {} ({}-{}). Res is {}", mid, lower_bound, upper_bound, res);
        if res == 0.0 {
            println!("Res 2: {}", mid);
            return;
        } else if res < 0.0 {
            upper_bound = mid;
        } else {
            lower_bound = mid;
        }

        nb_iter += 1;
        if nb_iter >= 100 {
            break;
        }
    }

}

fn iter(nodes: HashMap<&str, RefCell<Node>>, mut queue: VecDeque<String>) -> f64 {
    while !queue.is_empty() {
        let name = queue.pop_front().unwrap();
        let node = nodes.get(&name as &str).unwrap().borrow();
        for (waiting, id) in node.waiting_list.iter() {
            let mut waiting_node = nodes.get(waiting as &str).unwrap().borrow_mut();
            waiting_node.waits_for -= 1;
            if waiting_node.waits_for == 0 {
                waiting_node.value = Some(match id {
                    0 => waiting_node.op.unwrap()(node.value.unwrap(), waiting_node.value.unwrap()),
                    _ => waiting_node.op.unwrap()(waiting_node.value.unwrap(), node.value.unwrap()),
                });
                queue.push_back(waiting.to_string());
            } else {
                waiting_node.value = node.value;
            }
        }
    }

    nodes.get("root").unwrap().borrow().value.unwrap()
}