#![allow(unused)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn process_part_1(filename: &str) -> Option<usize> {
    if let Ok(lines) = read_lines(filename) {
        let total_fully_contained_pairs = lines
            .filter(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
            .filter(|l| {
                if let Ok(line) = l {
                    let pairs: Vec<i32> = line
                        .split(',')
                        .collect::<Vec<&str>>()
                        .iter()
                        .flat_map(|s| s.split('-'))
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

                    debug_assert!(pairs.len() == 4);

                    return (pairs[0] <= pairs[2] && pairs[1] >= pairs[3])
                        || (pairs[2] <= pairs[0] && pairs[3] >= pairs[1]);
                }

                false
            })
            .count();

        return Some(total_fully_contained_pairs);
    }

    None
}

pub fn process_part_2(filename: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(filename) {
        return Some(0);
    }

    None
}
