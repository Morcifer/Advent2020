use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn get_file_path(is_test: bool, day: u32) -> String {
    let sub_folder = if is_test { "test" } else { "real" };
    format!("./data/{sub_folder}/day{day}.txt")
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
