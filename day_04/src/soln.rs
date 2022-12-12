use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_lines(lines: Lines<BufReader<File>>) -> Vec<Vec<i32>> {
    let line_pairs = lines
        .filter(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
        .map(|l| {
            let pairs: Vec<i32> = l
                .unwrap()
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .flat_map(|s| s.split('-'))
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            debug_assert!(pairs.len() == 4);

            return pairs;
        })
        .collect::<Vec<Vec<i32>>>();

    line_pairs
}

pub fn process_part_1(filename: &str) -> Option<usize> {
    if let Ok(lines) = read_lines(filename) {
        let total_fully_contained_pairs = process_lines(lines)
            .iter()
            .filter(|v| (v[0] <= v[2] && v[1] >= v[3]) || (v[2] <= v[0] && v[3] >= v[1]))
            .count();

        return Some(total_fully_contained_pairs);
    }

    None
}

pub fn process_part_2(filename: &str) -> Option<usize> {
    if let Ok(lines) = read_lines(filename) {
        let total_fully_contained_pairs = process_lines(lines)
            .iter()
            .filter(|v| {
                (v[0] <= v[2] && (v[1] >= v[3] || v[1] >= v[2]))
                    || (v[2] <= v[0] && (v[3] >= v[1] || v[3] >= v[0]))
            })
            .count();

        return Some(total_fully_contained_pairs);
    }

    None
}
