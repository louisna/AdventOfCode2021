#[macro_use]
extern crate log;

use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    rate: i64,
    neighs: Vec<usize>,
}

fn main() {
    env_logger::init();
    let content = include_str!("../../inputs/16.txt");

    let mut name_to_id = HashMap::new();
    for (i, line) in content.split('\n').enumerate() {
        let name = line.split(' ').skip(1).next().unwrap();
        name_to_id.insert(name.to_string(), i);
    }

    let mut start = 0;

    let nodes: Vec<_> = content.split('\n').enumerate().map(|(i, line)| {
        let mut split = line.split(' ').skip(1);
        let node_name = split.next().unwrap();
        if node_name == "AA" {
            start = i;
        }
        let rate = split.skip(2).next().unwrap();
        let rate = rate[..rate.len() - 1].split('=').skip(1).next().unwrap().parse::<i64>().unwrap();
        let neighs: Vec<_> = match line.split("valves ").skip(1).next() {
            Some(v) => v.split(", ").map(|node_name| *name_to_id.get(node_name).unwrap()).collect(),
            None => line.split("to valve ").skip(1).next().unwrap().split(", ").map(|node_name| *name_to_id.get(node_name).unwrap()).collect(),
        };
        Node {
            rate,
            neighs,
        }
    }).collect();

    // Key: (current node, open valve bitmap, remaining time). Value: flow.
    // let mut memo: HashMap<(usize, u64, i64), i64> = HashMap::new();

    // let res = explore(&nodes[..], start, 0, 30, 0, &mut memo);
    // debug!("Res: {}", res);

    // Key: (current node 1, current node 2, open valve bitmap, remaining time). Value: flow.
    let mut memo = HashMap::new();
    let res = explore2(&nodes[..], start, start, 0, 26, 0, &mut memo, true, 0, 0);
    debug!("Map: {:?}", memo);
    println!("Res 2: {}", res);

}

fn explore2(nodes: &[Node], current_node_1: usize, current_node_2: usize, open_valves: u64, remaining_time: i64, current_flow: i64, memo: &mut HashMap<(usize, usize, u64, i64), i64>, first_move: bool, tmp_value: i64, tmp_active: i32) -> i64 {
    debug!("First has to play: {}", first_move);// No remaining time so we cannot do anything more.
    if remaining_time <= 0 {
        return 0;
    }
    
    // Already explored node.
    if memo.contains_key(&(current_node_1, current_node_2, open_valves, remaining_time)) {
        return *memo.get(&(current_node_1, current_node_2, open_valves, remaining_time)).unwrap();
    }

    let moving_node = if first_move { current_node_1 } else { current_node_2 };

    // Can open the valve if not null and not already done.
    let val_open = if nodes[moving_node].rate > 0 && (open_valves & (1 << moving_node)) == 0 {
        if first_move {
            debug!("First open valve at {} ({})", moving_node, nodes[moving_node].rate);
            explore2(nodes, moving_node, current_node_2, open_valves, remaining_time, current_flow, memo, false, nodes[moving_node].rate, moving_node as i32)
        } else {
            if tmp_active == current_node_2 as i32 {
                0
            } else {
                explore2(nodes, current_node_1, moving_node, open_valves | (1 << moving_node) | (1 << tmp_active), remaining_time - 1, current_flow + nodes[moving_node].rate + tmp_value, memo, true, 0, -1)
            }
        }
    } else {
        0
    };

    // Can move to neighbours.
    let val_move = nodes[moving_node].neighs.iter().map(|&neigh| {
        if first_move {
            debug!("At {}, First move from {} to {}", remaining_time, moving_node, neigh);
            explore2(nodes, neigh, current_node_2, open_valves, remaining_time, current_flow, memo, false, 0, -1)
        } else {
            debug!("At {}, Second move from {} to {}", remaining_time, moving_node, neigh);
            explore2(nodes, current_node_1, neigh, open_valves | (1 << tmp_active), remaining_time - 1, current_flow + tmp_value, memo, true, 0, -1)
        }
    }).max().unwrap();

    if !first_move {
        debug!("Best flow is {}+{}+{}", current_flow, val_open, tmp_value);
        memo.insert((current_node_1, current_node_2, open_valves | (1 << tmp_active), remaining_time), current_flow + val_open.max(val_move));
        return current_flow + val_open.max(val_move)
    } else {
        return val_move.max(val_open)
    }
}

fn explore(nodes: &[Node], current_node: usize, open_valves: u64, remaining_time: i64, current_flow: i64, memo: &mut HashMap<(usize, u64, i64), i64>) -> i64 {
    // No remaining time so we cannot do anything more.
    if remaining_time <= 0 {
        return 0;
    }
    
    // Already explored node.
    if memo.contains_key(&(current_node, open_valves, remaining_time)) {
        return *memo.get(&(current_node, open_valves, remaining_time)).unwrap();
    }

    // Can open the valve if not null and not already done.
    let val_open = if nodes[current_node].rate > 0 && (open_valves & (1 << current_node)) == 0 {
        explore(nodes, current_node, open_valves | (1 << current_node), remaining_time - 1, current_flow + nodes[current_node].rate, memo)
    } else {
        0
    };

    // Can move to neighbours.
    let val_move = nodes[current_node].neighs.iter().map(|&neigh| {
        explore(nodes, neigh, open_valves, remaining_time - 1, current_flow, memo)
    }).max().unwrap();

    memo.insert((current_node, open_valves, remaining_time), current_flow + val_open.max(val_move));

    current_flow + val_open.max(val_move)
}