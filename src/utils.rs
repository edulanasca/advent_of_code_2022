use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

pub fn read_file(day: &str) -> Option<BufReader<File>> {
    let mut path = PathBuf::from("/Users/edujlac/Documents/Projects/advent_of_code_2022/assets");
    path.push(format!("day_{}_input.txt", day));

    match File::open(path) {
        Ok(file) => Some(BufReader::new(file)),
        Err(e) => {
            println!("Couldn't open file {}", e);
            return None;
        }
    }
}
