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
                    match index {
                        1 => stacks[0].insert(0, *c),
                        5 => stacks[1].insert(0, *c),
                        9 => stacks[2].insert(0, *c),
                        _ => unreachable!(),
                    }
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
        .map(|order| get_orders(order))
        .for_each(|(count, from, to)| {
            //stacks[to].push(stacks[from].);
        })
}

pub fn process_part_1(filename: &str) -> &str {
    let raw = read_input_file(filename);
    let (crates, orders) = raw.split_once("\n\n").unwrap();

    let mut initial_stacks = create_initial_stacks(crates);
    process_orders(&mut initial_stacks, orders);

    ""
}

pub fn process_part_2(filename: &str) -> &str {
    ""
}
