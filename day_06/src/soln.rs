use std::collections::HashSet;
use std::fs;

fn read_input_file(filename: &str) -> String {
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Error reading the file {}", filename))
        .replace('\r', "")
}

fn combine(res: &Vec<usize>) -> String {
    res.iter()
        .fold("".to_owned(), |mut accu, curr| {
            accu.push_str(curr.to_string().as_str());
            accu.push(' ');
            accu
        })
        .trim_end()
        .to_string()
}

fn tune(raw: String, sequence_length: usize) -> String {
    let res = raw
        .lines()
        .map(|line| {
            let mut n = 0;
            let chars = line.chars().collect::<Vec<char>>();

            for i in 0..chars.len() - sequence_length {
                let set: HashSet<char> =
                    HashSet::from_iter(chars[i..i + sequence_length].to_owned());

                if set.len() == sequence_length {
                    n = i;
                    break;
                }
            }

            n + sequence_length
        })
        .collect();

    combine(&res)
}

pub fn process_part_1(filename: &str) -> String {
    let sequence_length = 4;
    let raw = read_input_file(filename);

    tune(raw, sequence_length)
}

pub fn process_part_2(filename: &str) -> String {
    let sequence_length = 14;
    let raw = read_input_file(filename);

    tune(raw, sequence_length)
}
