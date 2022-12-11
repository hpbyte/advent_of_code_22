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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn get(h: &str) -> Move {
        match h {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissor,
            _ => unreachable!(),
        }
    }

    fn get_counter_move(m: &Move, is_win: bool) -> Move {
        if is_win {
            return match m {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissor,
                Move::Scissor => Move::Rock,
            };
        }

        match m {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
        }
    }

    fn get_score(m: &Move) -> i32 {
        match m {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn get_desired_outcome(h: &str) -> Outcome {
        match h {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!(),
        }
    }

    fn get(moves: &(Move, Move)) -> Outcome {
        match moves {
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissor)
            | (Move::Scissor, Move::Rock) => Outcome::Win,
            (opponent, player) if opponent == player => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }

    fn get_score(outcome: &Outcome) -> i32 {
        match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

fn get_desired_player_move(desired_outcome: &Outcome, opponent_move: &Move) -> Move {
    match desired_outcome {
        Outcome::Win => Move::get_counter_move(opponent_move, true),
        Outcome::Lose => Move::get_counter_move(opponent_move, false),
        Outcome::Draw => *opponent_move,
    }
}

pub fn process_part_1(filename: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(filename) {
        let total_score = lines
            .filter(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
            .map(|l| {
                l.unwrap()
                    .split_once(' ')
                    .map(|(opponent, player)| (Move::get(opponent), Move::get(player)))
                    .unwrap()
            })
            .fold(0, |acc, moves| {
                acc + Move::get_score(&moves.1) + Outcome::get_score(&Outcome::get(&moves))
            });

        return Some(total_score);
    }

    None
}

pub fn process_part_2(filename: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(filename) {
        let total_score = lines
            .filter(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
            .map(|l| {
                l.unwrap()
                    .split_once(' ')
                    .map(|(opponent, desired_outcome)| {
                        let outcome = Outcome::get_desired_outcome(desired_outcome);
                        let player_move = get_desired_player_move(&outcome, &Move::get(opponent));

                        (outcome, player_move)
                    })
                    .unwrap()
            })
            .fold(0, |acc, (outcome, player)| {
                acc + Move::get_score(&player) + Outcome::get_score(&outcome)
            });

        return Some(total_score);
    }

    None
}
