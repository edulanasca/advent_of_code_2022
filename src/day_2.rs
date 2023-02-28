use std::io::{BufRead, BufReader};
use crate::utils;

pub fn rock_paper_scissors() {
    let file = match utils::read_file("day_2_input.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Couldn't open file {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut score = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(' ').collect();
        let op_move = moves[0];
        let my_move = moves[1];

        match op_move {
            "A" => { // Rock
                match my_move {
                    "X" => score += 1 + 3, // Rock
                    "Y" => score += 2 + 6, // Paper
                    "Z" => score += 3 + 0, // Scissors,
                    _ => continue
                }
            },
            "B" => { // Paper
                match my_move {
                    "X" => score += 1 + 0, // Rock
                    "Y" => score += 2 + 3, // Paper
                    "Z" => score += 3 + 6, // Scissors
                    _ => continue
                }
            },
            "C" => { // Scissors
                match my_move {
                    "X" => score += 1 + 6, // Rock
                    "Y" => score += 2 + 0, // Paper
                    "Z" => score += 3 + 3, // Scissors
                    _ => continue
                }
            },
            _ => continue,
        }
    }

    println!("Total score {}", score);
}
