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

fn get_common_item(batch: Vec<&String>) -> char {
    let mut set: HashSet<char> = HashSet::new();

    batch.iter().for_each(|x| set.extend(x.chars()));
    batch.iter().for_each(|x| set.retain(|y| x.contains(*y)));

    // there must 1 match that is common
    debug_assert!(set.len() == 1);

    *set.iter().next().unwrap()
}

pub fn process_part_1(filename: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(filename) {
        let total_priority = lines
            .filter(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
            .map(|l| {
                if let Ok(line) = l {
                    let (first, second) = line.split_at(line.len() >> 1);
                    let common_item = get_common_item(vec![&first.to_string(), &second.to_string()]);
                    return get_priority(&common_item);
                }

                0
            })
            .sum();

        return Some(total_priority);
    }

    None
}

pub fn process_part_2(filename: &str) -> Option<i32> {
    const BATCH_SIZE: usize = 3;

    if let Ok(lines) = read_lines(filename) {
        let total_priority = lines
            .filter(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
            .collect::<Vec<_>>().chunks(BATCH_SIZE)
            .map(|chunk| {
                let batch = chunk
                    .iter()
                    .map(|line| line.as_ref().unwrap())
                    .collect::<Vec<&String>>();
                let common_item = get_common_item(batch);
                return get_priority(&common_item);
            })
            .sum();

        return Some(total_priority);
    }

    None
}
