use std::fs::File;
use std::io::Result;
use std::path::PathBuf;

pub fn read_file(file_name: &str) -> Result<File> {
    let mut path = PathBuf::from("/Users/edujlac/Documents/Projects/advent_of_code_2022/assets");
    path.push(file_name);
    File::open(path)

}
