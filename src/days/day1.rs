use std::collections::HashMap;

use crate::days::util;

pub(crate) fn part1() {
    println!("Day 1 Part 1");
    let mut elf_num = 0;
    let mut elf_map: HashMap<i32, i32> = HashMap::new();
    if let Ok(lines) = util::read_lines("resources\\in1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if line.is_empty() {
                elf_num += 1;
                continue;
            }
            let calories: i32 = line.parse().unwrap();
            elf_map
                .entry(elf_num)
                .and_modify(|e| *e += calories)
                .or_insert(calories);
        }
    }

    let max_key = elf_map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| k)
        .unwrap();
    println!(
        "Elf {} with calories {}",
        max_key,
        elf_map.get(max_key).unwrap()
    );
}

pub(crate) fn part2() {
    println!("Day 1 Part 2");
    let mut elf_num = 0;
    let mut elf_map: HashMap<i32, i32> = HashMap::new();
    if let Ok(lines) = util::read_lines("resources\\in1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if line.is_empty() {
                elf_num += 1;
                continue;
            }
            let calories: i32 = line.parse().unwrap();
            elf_map
                .entry(elf_num)
                .and_modify(|e| *e += calories)
                .or_insert(calories);
        }
    }
    let mut calorie_sum = 0;
    for _ in 0..3 {
        let mut elf_num = 0;
        let mut max_val = 0;
        for (key, value) in elf_map.iter() {
            if value > &max_val {
                elf_num = *key;
                max_val = *value;
            }
        }
        let num_calories = elf_map.remove(&elf_num).unwrap();
        println!("Elf num {} with calories {}", elf_num, num_calories);
        calorie_sum += num_calories;
    }
    println!("Total Calories: {}", calorie_sum);
}
