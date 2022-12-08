use crate::days::util;
use std::collections::VecDeque;
use std::convert::TryFrom;

const NUM_STACKS: usize = 9;
pub(crate) fn part1() {
    println!("Day 5 Part 1");
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..NUM_STACKS {
        stacks.push(VecDeque::<char>::new())
    }
    if let Ok(lines) = util::read_lines("resources\\in5.txt") {
        for line in lines.flatten() {
            if line.is_empty() {
                continue;
            };
            if line.starts_with("move") {
                let mut split = line.split("move ");
                split.next();
                let num_to_move: i32 = split
                    .next()
                    .unwrap()
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                let mut split = line.split("from ");
                split.next();
                let mut start: i32 = split
                    .next()
                    .unwrap()
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                let mut split = line.split("to ");
                split.next();
                let mut end: i32 = split
                    .next()
                    .unwrap()
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                start -= 1;
                end -= 1;
                for _ in 0..num_to_move {
                    let crate_to_move =
                        stacks[usize::try_from(start).unwrap()].pop_front().unwrap();
                    stacks[usize::try_from(end).unwrap()].push_front(crate_to_move);
                }
            } else if line.contains('[') {
                let mut line_remaining: &str = &line;
                let mut crate_chars: &str;
                for (i, stack) in stacks.iter_mut().enumerate().take(NUM_STACKS) {
                    if i != NUM_STACKS - 1 {
                        (crate_chars, line_remaining) = line_remaining.split_at(3);
                    } else {
                        crate_chars = line_remaining;
                    }

                    line_remaining = &line_remaining[1..line_remaining.len()];
                    if crate_chars.starts_with('[') {
                        stack.push_back(crate_chars.chars().nth(1).unwrap())
                    }
                }
            }
        }
        let mut crate_str: String = String::new();
        for vec in stacks {
            crate_str.push(vec[0]);
        }
        println!("Top crates: {}", crate_str);
    }
}

pub(crate) fn part2() {
    println!("Day 5 Part 2");

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..NUM_STACKS {
        stacks.push(VecDeque::<char>::new())
    }
    if let Ok(lines) = util::read_lines("resources\\in5.txt") {
        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        for _ in 0..NUM_STACKS {
            stacks.push(VecDeque::<char>::new())
        }
        for line in lines.flatten() {
            if line.is_empty() {
                continue;
            };
            if line.starts_with("move") {
                let mut split = line.split("move ");
                split.next();
                let num_to_move: i32 = split
                    .next()
                    .unwrap()
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                let mut split = line.split("from ");
                split.next();
                let mut start: i32 = split
                    .next()
                    .unwrap()
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                let mut split = line.split("to ");
                split.next();
                let mut end: i32 = split
                    .next()
                    .unwrap()
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                start -= 1;
                end -= 1;

                let mut from_stack = stacks[usize::try_from(start).unwrap()].to_owned();
                stacks[usize::try_from(start).unwrap()] =
                    from_stack.split_off(usize::try_from(num_to_move).unwrap());

                from_stack.make_contiguous().reverse();
                for value in from_stack {
                    stacks[usize::try_from(end).unwrap()].push_front(value)
                }
            } else if line.contains('[') {
                let mut line_remaining: &str = &line;
                let mut crate_chars: &str;
                for (i, stack) in stacks.iter_mut().enumerate().take(NUM_STACKS) {
                    if i != NUM_STACKS - 1 {
                        (crate_chars, line_remaining) = line_remaining.split_at(3);
                    } else {
                        crate_chars = line_remaining;
                    }

                    line_remaining = &line_remaining[1..line_remaining.len()];
                    if crate_chars.starts_with('[') {
                        stack.push_back(crate_chars.chars().nth(1).unwrap())
                    }
                }
            }
        }
        let mut crate_str: String = String::new();
        for vec in stacks {
            crate_str.push(vec[0]);
        }
        println!("Top crates: {}", crate_str);
    }
}
