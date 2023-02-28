use std::io::{BufReader, BufRead};

use crate::utils;

pub fn calorie_counting() {
    let file = match utils::read_file("day_1_input.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Couldn't open file {}", e);
            return;
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
    println!("Top three elfs carry in total {} calories.", calories[len - 1] + calories[len -2] + calories[len -3]);
    println!("The elf with the most calories is carrying {} calories.", calories.last().unwrap());
}
