#![allow(unused)]

use std::fs;

fn read_input_file(filename: &str) -> String {
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Error reading the file {}", filename))
        .replace('\r', "")
}

fn create_initial_stacks(inital_crates: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![vec![], vec![], vec![]];

    inital_crates
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .for_each(|line_chars| {
            for (index, c) in line_chars.iter().enumerate() {
                if c.is_alphabetic() {
                    let i = (index + 2) % 3;
                    stacks[i].insert(0, *c);
                }
            }
        });

    stacks
}

fn get_orders(order: &str) -> (usize, usize, usize) {
    let order = order
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    debug_assert!(order.len() == 3);

    (order[0], order[1], order[2])
}

fn process_orders(stacks: &mut Vec<Vec<char>>, orders: &str) {
    orders
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|order| get_orders(order))
        .for_each(|(count, from, to)| {
            for _ in 0..count {
                let popped = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(popped);
            }
        })
}

pub fn process_part_1(filename: &str) -> String {
    let raw = read_input_file(filename);
    let (crates, orders) = raw.split_once("\n\n").unwrap();

    let mut stacks = create_initial_stacks(crates);
    process_orders(&mut stacks, orders);

    let final_top_crates = stacks
        .iter()
        .fold("".to_string(), |mut accu, stack| {
            let top_crate = stack.last().unwrap().to_string();
            accu.push_str(top_crate.as_str());

            accu
        });

    final_top_crates
}

pub fn process_part_2(filename: &str) -> &str {
    ""
}
