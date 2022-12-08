use crate::days::util;
use std::collections::{HashMap, HashSet};
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub(crate) fn part1() {
    println!("Day 3 Part 1");
    let priority_map = get_priorities();
    if let Ok(lines) = util::read_lines("resources\\in3.txt") {
        let mut priorities_sum = 0;
        for line in lines.flatten() {
            let line_length = line.chars().count();
            let (sack1, sack2) = line.split_at(line_length / 2);
            let mut matching_char: char = ' ';
            for character in sack1.chars() {
                if sack2.contains(character) {
                    matching_char = character;
                    break;
                }
            }
            priorities_sum += priority_map.get(&matching_char).unwrap();
        }
        println!("Sum of all priorities is: {}", priorities_sum);
    }
}

pub(crate) fn part2() {
    println!("Day 3 Part 2");
    let priority_map = get_priorities();
    if let Ok(lines) = util::read_lines("resources\\in3.txt") {
        let mut priorities_sum = 0;
        let mut vec: Vec<String> = Vec::new();
        for line in lines.flatten() {
            vec.push(line);
            if vec.len() == 3 {
                let mut matching_chars: HashSet<char> = HashSet::new();
                let sack1 = &vec[0];
                let sack2 = &vec[1];
                let sack3 = &vec[2];
                for character in sack1.chars() {
                    if sack2.contains(character) {
                        matching_chars.insert(character);
                    }
                }
                for character in matching_chars {
                    if sack3.contains(character) {
                        priorities_sum += priority_map.get(&character).unwrap();
                    }
                }
                vec.clear();
            }
        }
        println!("Sum of all priorities is: {}", priorities_sum);
    }
}

fn get_priorities() -> HashMap<char, i32> {
    let mut priority_map: HashMap<char, i32> = HashMap::new();
    let mut priority_val = 1;
    for character in ALPHABET.chars() {
        priority_map.insert(character, priority_val);
        priority_val += 1;
    }
    priority_map
}
