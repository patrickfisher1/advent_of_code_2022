use std::ops::Range;

use crate::days::util;

pub(crate) fn part1() {
    println!("Day 8 Part 1");

    let mut grid: Vec<Vec<u32>> = Vec::new();
    let mut grid_size: usize = 0;
    if let Ok(lines) = util::read_lines("resources\\in8.txt") {
        for line in lines {
            if let Ok(ip) = line {
                grid_size = ip.len();
                let numbers: Vec<u32> = ip.chars().map(|x| x.to_digit(10).unwrap()).collect();
                grid.push(numbers);
            }
        }
    }

    let mut num_trees_visible: i32 = 0;
    for (row_index, row) in grid.iter().enumerate() {
        if row_index == 0 || row_index == grid_size {
            num_trees_visible += grid_size as i32;
            continue;
        }
        for (col_index, height) in row.iter().enumerate() {
            // tree is on edge
            if col_index == 0 || col_index == grid_size {
                num_trees_visible += 1;
            } else {
                let mut column: Vec<u32> = Vec::new();
                for i in 0..grid_size {
                    column.push(grid[i][col_index]);
                }

                let visible_above = &column[0..row_index]
                    .iter()
                    .all(|t_height| t_height < height);
                let visible_below = &column[row_index + 1..grid_size]
                    .iter()
                    .all(|t_height| t_height < height);
                let visible_left = &row[0..col_index].iter().all(|t_height| t_height < height);
                let visible_right = &row[col_index + 1..grid_size]
                    .iter()
                    .all(|t_height| t_height < height);

                if *visible_above || *visible_below || *visible_left || *visible_right {
                    num_trees_visible += 1;
                }
            }
        }
    }
    println!("{} trees visible", num_trees_visible);
}

fn get_view_distance(line: &Vec<u32>, range: impl Iterator<Item = usize>, height: &u32) -> usize {
    let mut count = 0;
    for i in range {
        count += 1;
        if &line[i] >= height {
            break;
        }
    }
    return count;
}

pub(crate) fn part2() {
    println!("Day 8 Part 2");

    let mut grid: Vec<Vec<u32>> = Vec::new();
    let mut grid_size: usize = 0;
    if let Ok(lines) = util::read_lines("resources\\in8.txt") {
        for line in lines {
            if let Ok(ip) = line {
                grid_size = ip.len();
                let numbers: Vec<u32> = ip.chars().map(|x| x.to_digit(10).unwrap()).collect();
                grid.push(numbers);
            }
        }
    }

    let mut max_scenic_score: usize = 0;
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, height) in row.iter().enumerate() {
            let mut column: Vec<u32> = Vec::new();
            for i in 0..grid_size {
                column.push(grid[i][col_index]);
            }
            let distance_up = get_view_distance(&column, (0..row_index).rev(), height);
            let distance_down = get_view_distance(&column, row_index + 1..grid_size, height);
            let distance_left = get_view_distance(&row, (0..col_index).rev(), height);
            let distance_right = get_view_distance(&row, col_index + 1..grid_size, height);

            let scenic_score = distance_up * distance_down * distance_left * distance_right;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    println!("Highest scenic score: {}", max_scenic_score);
}
