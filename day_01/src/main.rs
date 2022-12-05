use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./test.input") {
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
                        },
                        Err(_e) => {
                            panic!("cannot convert to an integer");
                        }
                    }
                }
            }
        }
        println!("The most calories being carried is {}", prev_max);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
