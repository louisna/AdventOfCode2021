use std::{cell::RefCell, collections::LinkedList};
use regex::Regex;

#[derive(Debug)]
struct Monkey {
    nb_inspect: i64,
    operand: Option<i64>,
    operation: String,
    true_throw: usize,
    false_throw: usize,
    divider: i64,
    objects: LinkedList<i64>,
}

fn main() {
    let content = include_str!("../../inputs/test.txt");

    let mut monkeys: Vec<RefCell<Monkey>> = content.split("\n\n").map(|monkey_str| {
        let mut monkey_iter = monkey_str.split("\n").skip(1);

        // Starting items.
        let objects: LinkedList<i64> = monkey_iter.next().unwrap().split(": ").skip(1).next().unwrap().split(", ").map(|o| o.parse().unwrap()).collect();

        // Operation.
        let re = Regex::new(r"Operation: new = old (\D) (\w*)").unwrap();
        let captures = re.captures(monkey_iter.next().unwrap()).unwrap();
        let mut captures = captures.iter().skip(1);

        let op = captures.next().unwrap().unwrap().as_str();
        let operand = captures.next().unwrap().unwrap().as_str();
        println!("OP={}, OPERAND={}", op, operand);
        let operand = match operand {
            "old" => None,
            digit => Some(digit.parse().unwrap()),
        };

        let divider: i64 = monkey_iter.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        let true_throw: usize = monkey_iter.next().unwrap().split(" ").last().unwrap().parse().unwrap();
        let false_throw: usize = monkey_iter.next().unwrap().split(" ").last().unwrap().parse().unwrap();

        RefCell::new(Monkey {
            nb_inspect: 0,
            operation: op.to_string(),
            operand,
            true_throw,
            false_throw,
            divider,
            objects,
        })

    }).collect();

    let modulo: i64 = i64::MAX;//monkeys.iter().map(|m| m.borrow().divider).product();

    for _i in 0..10_000 {
        for (mkid, monkey) in monkeys.iter().enumerate() {
            let mut monkey_borrow = monkey.borrow_mut();
            while let Some(v) = monkey_borrow.objects.pop_front() {
                let operand = match monkey_borrow.operand {
                    None => v,
                    Some(i) => i,
                };
                let value = match monkey_borrow.operation.as_str() {
                    "+" => v + operand,//((v % modulo) + (operand % modulo)) % modulo,
                    "*" => v * operand,//((v % modulo) * (operand % modulo)) % modulo,
                    _ => panic!("Unknown operation"),
                };

                // Borred monkey.
                // let value = value / 3;

                let next_monkey_idx = if value % monkey_borrow.divider == 0 {
                    monkey_borrow.true_throw
                } else {
                    monkey_borrow.false_throw
                };

                let mut next_monkey = monkeys[next_monkey_idx].borrow_mut();
                next_monkey.objects.push_back(value);

                // We processed an item.
                monkey_borrow.nb_inspect += 1;
            }
        }
    }

    // Sort according to number of operations.
    println!("The monkeys: {:?}", monkeys);
    monkeys.sort_by(|a, b| (-a.borrow().nb_inspect).cmp(&-(b.borrow().nb_inspect)));
    let res = monkeys[0].borrow().nb_inspect * monkeys[1].borrow().nb_inspect;
    println!("Res 1: {}", res);
}