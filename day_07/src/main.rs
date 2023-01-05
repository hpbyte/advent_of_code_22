use std::env;

mod soln;

use soln::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Part 1: the total size: {:?}", process_part_1(filename));

    println!("Part 2: the total size: {:?}", process_part_2(filename));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_work() {
        let ans = process_part_1("./sample.input");
        assert_eq!(ans, 95437);
    }

    #[test]
    fn part_2_should_work() {
        let ans = process_part_2("./sample.input");
        assert_eq!(ans, 1770595);
    }
}
