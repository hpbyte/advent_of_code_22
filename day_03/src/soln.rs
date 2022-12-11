#![allow(unused)]

use std::collections::HashSet;
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

fn get_priority(c: &char) -> i32 {
    if c.is_lowercase() {
        // a-z is 1 thru 26
        return *c as i32 - 'a' as i32 + 1;
    }

    // A-Z is 27 thru 52
    *c as i32 - 'A' as i32 + 27
}

fn get_common_item(str1: &str, str2: &str) -> Option<char> {
    let (shorter, longer) = if str1.len() > str2.len() {
        (str2, str1)
    } else {
        (str1, str2)
    };
    let set: HashSet<char> = shorter.chars().collect();

    for c in longer.chars() {
        if set.contains(&c) {
            return Some(c);
        }
    }

    None
}

pub fn process_part_1(filename: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(filename) {
        let total_priority = lines
            .filter(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
            .map(|l| {
                if let Ok(line) = l {
                    let (first, second) = line.split_at(line.len() >> 1);
                    if let Some(common_item) = get_common_item(first, second) {
                        return get_priority(&common_item);
                    }
                }

                0
            })
            .sum();

        return Some(total_priority);
    }

    None
}

pub fn process_part_2(filename: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(filename) {
        return Some(0);
    }

    None
}
