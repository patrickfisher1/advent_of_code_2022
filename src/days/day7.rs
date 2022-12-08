use crate::days::util;

struct FileOrFolder {
    is_file: bool,
    size: i32,
    name: String,
    files: Vec<FileOrFolder>,
}

pub(crate) fn part1() {
    println!("Day 7 Part 1");
    let mut directory_list: Vec<FileOrFolder> = Vec::new();
    let mut current_folder: &str;
    if let Ok(lines) = util::read_lines("resources\\in7ex.txt") {
        for line in lines.flatten() {
            //Command given
            if line.starts_with('$') {
                let (_, command) = line.split_at(2);

                let mut split = command.split_ascii_whitespace();

                let command = split.next().unwrap();
                match command {
                    "cd" => {
                        let folder = split.next().unwrap();
                        directory_list.push(FileOrFolder {
                            is_file: false,
                            size: 0,
                            name: folder.to_string(),
                            files: Vec::new(),
                        });
                        current_folder = folder;
                        println!("Go to folder: {}", folder);
                    }
                    "ls" => {} //Do nothing
                    _ => {}
                }
            //Below branches are reached after an ls occurs
            } else if line.starts_with("dir") {
                let (_, folder) = line.split_once(' ').unwrap();
                println!("Folder named: {}", folder);
            } else if line.starts_with(char::is_numeric) {
                let (size, file) = line.split_once(' ').unwrap();
                println!("File {} of size {}", file, size);
            }
        }
    }
}

pub(crate) fn part2() {
    println!("Day 7 Part 2");

    if let Ok(lines) = util::read_lines("resources\\in7ex.txt") {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}
