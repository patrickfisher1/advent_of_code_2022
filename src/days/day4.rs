use std::collections::{HashMap, HashSet};
use crate::days::util;

pub(crate) fn part1 () {
    println!("Day 4 Part 1");
    if let Ok(lines) = util::read_lines("resources\\in4.txt" ) {
        let mut contained_pairs = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split(',');
                let (elf1, elf2) = (split.next().unwrap(), split.next().unwrap());

                let mut split = elf1.split('-');
                let (elf1_range_start, elf1_range_end) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

                let mut split = elf2.split('-');
                let (elf2_range_start, elf2_range_end) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());
                
                if elf1_range_start <= elf2_range_start && elf1_range_end >= elf2_range_end || elf2_range_start <= elf1_range_start && elf2_range_end >= elf1_range_end {
                    contained_pairs += 1;
                }
            }
        }
        println!("Number of contained pairs: {}", contained_pairs);
    }
}

pub(crate) fn part2() {
    println!("Day 4 Part 2");
    if let Ok(lines) = util::read_lines("resources\\in4.txt" ) {
        let mut overlapping_pairs = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut split = ip.split(',');
                let (elf1, elf2) = (split.next().unwrap(), split.next().unwrap());

                let mut split = elf1.split('-');
                let (elf1_range_start, elf1_range_end) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

                let mut split = elf2.split('-');
                let (elf2_range_start, elf2_range_end) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

                let mut elf1_range = elf1_range_start .. elf1_range_end+1;

                let mut elf2_range = elf2_range_start .. elf2_range_end+1;
                
                if elf1_range.any(|x| elf2_range.contains(&x)) {
                    overlapping_pairs += 1;
                }
            }
        }
        println!("Number of overlapping pairs: {}", overlapping_pairs);
    }
}