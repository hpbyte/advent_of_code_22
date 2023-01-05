use std::fs;

mod file_tree;

use file_tree::create_file_tree;

fn read_input_file(filename: &str) -> String {
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Error reading the file {}", filename))
        .replace('\r', "")
}

pub fn process_part_1(filename: &str) -> u32 {
    let raw = read_input_file(filename);
    let file_tree = create_file_tree(raw);

    file_tree.print();

    file_tree
        .flat_children()
        .iter()
        .filter(|file| file.is_dir && file.size <= 100_000)
        .fold(0, |accu, curr| accu + curr.size)
}

pub fn process_part_2(filename: &str) -> u32 {
    0
}
