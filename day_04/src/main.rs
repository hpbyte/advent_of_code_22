use std::env;

mod soln;

use soln::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    if let Some(ans) = process_part_1(filename) {
        println!("Part 1 : The total number of assignment pairs: {}", ans);
    }

    if let Some(ans) = process_part_2(filename) {
        println!("Part 2 : The total number of assignment pairs: {}", ans);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_work() {
        let ans = process_part_1("./sample.input").unwrap();
        assert_eq!(ans, 2);
    }

    #[test]
    fn part_2_should_work() {
        let ans = process_part_2("./sample.input").unwrap();
        assert_eq!(ans, 4);
    }
}
