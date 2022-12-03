use crate::days::util;

const ROCK_POINTS: i32 = 1;
const PAPER_POINTS: i32 = 2;
const SCISSORS_POINTS: i32 = 3;
const WIN_POINTS: i32 = 6;
const DRAW_POINTS: i32 = 3;
const LOSE_POINTS: i32 = 0;

const OPP_ROCK: &str = "A";
const OPP_PAPER: &str = "B";
const OPP_SCISSORS: &str = "C";

const ME_ROCK: &str = "X";
const ME_PAPER: &str = "Y";
const ME_SCISSORS: &str = "Z";

const SHOULD_LOSE: &str = "X";
const SHOULD_DRAW: &str = "Y";
const SHOULD_WIN: &str = "Z";

pub(crate) fn part1() {
    println!("Day 2 Part 1");
    if let Ok(lines) = util::read_lines("resources\\in2.txt") {
        let mut total_score = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split(' ');
                let opponent_move = split.next().unwrap();
                let my_move = split.next().unwrap();
                // println!("Opponent move: {} - My move: {}", opponent_move, my_move);
                let round_score = match (opponent_move, my_move) {
                    (OPP_ROCK, ME_ROCK) => ROCK_POINTS + DRAW_POINTS,
                    (OPP_ROCK, ME_PAPER) => PAPER_POINTS + WIN_POINTS,
                    (OPP_ROCK, ME_SCISSORS) => SCISSORS_POINTS + LOSE_POINTS,

                    (OPP_PAPER, ME_ROCK) => ROCK_POINTS + LOSE_POINTS,
                    (OPP_PAPER, ME_PAPER) => PAPER_POINTS + DRAW_POINTS,
                    (OPP_PAPER, ME_SCISSORS) => SCISSORS_POINTS + WIN_POINTS,

                    (OPP_SCISSORS, ME_ROCK) => ROCK_POINTS + WIN_POINTS,
                    (OPP_SCISSORS, ME_PAPER) => PAPER_POINTS + LOSE_POINTS,
                    (OPP_SCISSORS, ME_SCISSORS) => SCISSORS_POINTS + DRAW_POINTS,
                    _ => {
                        println!("No matching case");
                        0
                    }
                };
                // println!("Round score: {}", round_score);
                total_score += round_score;
            }
        }
        println!("Final score: {}", total_score);
    }
}

pub(crate) fn part2() {
    println!("Day 2 Part 2");
    if let Ok(lines) = util::read_lines("resources\\in2.txt") {
        let mut total_score = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split(' ');
                let opponent_move = split.next().unwrap();
                let my_move = split.next().unwrap();
                // println!("Opponent move: {} - My move: {}", opponent_move, my_move);
                let round_score = match (opponent_move, my_move) {
                    (OPP_ROCK, SHOULD_LOSE) => SCISSORS_POINTS + LOSE_POINTS,
                    (OPP_ROCK, SHOULD_DRAW) => ROCK_POINTS + DRAW_POINTS,
                    (OPP_ROCK, SHOULD_WIN) => PAPER_POINTS + WIN_POINTS,

                    (OPP_PAPER, SHOULD_LOSE) => ROCK_POINTS + LOSE_POINTS,
                    (OPP_PAPER, SHOULD_DRAW) => PAPER_POINTS + DRAW_POINTS,
                    (OPP_PAPER, SHOULD_WIN) => SCISSORS_POINTS + WIN_POINTS,

                    (OPP_SCISSORS, SHOULD_LOSE) => PAPER_POINTS + LOSE_POINTS,
                    (OPP_SCISSORS, SHOULD_DRAW) => SCISSORS_POINTS + DRAW_POINTS,
                    (OPP_SCISSORS, SHOULD_WIN) => ROCK_POINTS + WIN_POINTS,
                    _ => {
                        println!("No matching case");
                        0
                    }
                };
                // println!("Round score: {}", round_score);
                total_score += round_score;
            }
        }
        println!("Final score: {}", total_score);
    }
}
