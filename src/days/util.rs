use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(crate) fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let current_dir = env::current_dir()?;
    let binding = current_dir.join(path);
    let filepath = binding.as_path();
    // println!("OPENING FILE {}", path.as_ref().display());
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
