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

pub fn process_part_1(filename: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(filename) {
        let mut prev_max = 0;
        let mut sum = 0;
        for line in lines {
            if let Ok(str) = line {
                if str.is_empty() {
                    if sum > prev_max {
                        prev_max = sum;
                    }
                    sum = 0;
                } else {
                    match str.parse::<i32>() {
                        Ok(num) => {
                            sum = sum + num;
                        }
                        Err(_e) => {
                            panic!("cannot convert to an integer");
                        }
                    }
                }
            }
        }
        return Some(prev_max);
    }

    None
}

pub fn process_part_2(filename: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        let mut sums: Vec<i32> = Vec::new();

        for line in lines {
            if let Ok(str) = line {
                if str.is_empty() {
                    sums.push(sum);
                    sum = 0;
                    continue;
                }

                match str.parse::<i32>() {
                    Ok(num) => {
                        sum = sum + num;
                    }
                    Err(_e) => {
                        panic!("cannot convert to an integer");
                    }
                }
            }
        }
        sums.sort_by(|a, b| b.cmp(a));
        return Some(sums.iter().take(3).sum());
    }

    None
}
