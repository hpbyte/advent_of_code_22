#![allow(unused)]

use std::fs;

fn read_input_file(filename: &str) -> String {
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Error reading the file {}", filename))
        .replace('\r', "")
}

fn create_initial_stacks(inital_crates: &str) -> Vec<Vec<char>> {
    let row_size = inital_crates.lines().count();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; row_size];

    inital_crates
        .lines()
        .filter(|x| !x.starts_with(" 1"))
        .map(|line| line.chars().collect::<Vec<char>>())
        .for_each(|line_chars| {
            for (index, c) in line_chars
                .iter()
                .enumerate()
                .filter(|x| x.1.is_ascii_uppercase())
            {
                stacks[(index as f32 / 4.).ceil() as usize - 1].insert(0, *c);
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

fn process_orders(stacks: &mut Vec<Vec<char>>, orders: &str, batched: bool) {
    orders
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|order| get_orders(order))
        .for_each(|(count, from, to)| {
            let mut batch_crates: Vec<char> = vec![];

            for _ in 0..count {
                if let Some(popped) = stacks[from - 1].pop() {
                    if batched {
                        batch_crates.insert(0, popped);
                    } else {
                        batch_crates.push(popped);
                    }
                }
            }

            stacks[to - 1].append(&mut batch_crates);
        })
}

pub fn process_part_1(filename: &str) -> String {
    let raw = read_input_file(filename);
    let (crates, orders) = raw.split_once("\n\n").unwrap();

    let mut stacks = create_initial_stacks(crates);
    process_orders(&mut stacks, orders, false);

    let final_top_crates = stacks.iter().fold("".to_string(), |mut accu, stack| {
        if let Some(top_crate) = stack.last() {
            accu.push_str(top_crate.to_string().as_str());
        }

        accu
    });

    final_top_crates
}

pub fn process_part_2(filename: &str) -> String {
    let raw = read_input_file(filename);
    let (crates, orders) = raw.split_once("\n\n").unwrap();

    let mut stacks = create_initial_stacks(crates);
    process_orders(&mut stacks, orders, true);

    let final_top_crates = stacks.iter().fold("".to_string(), |mut accu, stack| {
        if let Some(top_crate) = stack.last() {
            accu.push_str(top_crate.to_string().as_str());
        }

        accu
    });

    final_top_crates
}
