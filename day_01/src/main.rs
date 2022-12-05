use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./test.input") {
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
                    },
                    Err(_e) => {
                        panic!("cannot convert to an integer");
                    }
                }
            }
        }
        sums.sort_by(|a, b| b.cmp(a));
        let sum: i32 = sums.iter().take(3).sum();
        println!("The top three most calories being carried are: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
