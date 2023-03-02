use std::io::{BufReader, BufRead};

use crate::utils;

pub fn calorie_counting() -> Option<(i32, i32)> {
    let file = match utils::read_file("day_1_input.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Couldn't open file {}", e);
            return None;
        }
    };

    let reader = BufReader::new(file);
    let mut calories = Vec::new();
    let mut accumulator = 0;

    for line in reader.lines() {
        match line.unwrap().parse::<i32>() {
            Ok(num) => accumulator += num,
            Err(_) => {
                // A emtpy line is being parse so the elf is done writing calories
                // Compare if the current elf (accumulator) has more calories than the current max

                calories.push(accumulator);
                accumulator = 0;
            }
        }
    }

    calories.sort();

    let len: usize = calories.len();
    Some((*calories.last().unwrap(), calories[len - 3..].iter().sum()))
}
