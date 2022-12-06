use crate::days::util;
use itertools::Itertools;

pub(crate) fn part1() {
    println!("Day 6 Part 1");
    if let Ok(lines) = util::read_lines("resources\\in6.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut char_buffer: Vec<char> = Vec::new();
                for (i, char) in ip.chars().enumerate() {
                    if char_buffer.len() == 4 {
                        if char_buffer.iter().all_unique() {
                            println!("First marker character after character {}", i);
                            return;
                        }
                        char_buffer.remove(0);
                    }
                    char_buffer.push(char);
                }
            }
        }
    }
}

pub(crate) fn part2() {
    println!("Day 6 Part 2");

    if let Ok(lines) = util::read_lines("resources\\in6.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut char_buffer: Vec<char> = Vec::new();
                for (i, char) in ip.chars().enumerate() {
                    if char_buffer.len() == 14 {
                        if char_buffer.iter().all_unique() {
                            println!("First marker character after character {}", i);
                            return;
                        }
                        char_buffer.remove(0);
                    }
                    char_buffer.push(char);
                }
            }
        }
    }
}
