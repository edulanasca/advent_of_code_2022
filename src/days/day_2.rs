use std::io::{BufRead, BufReader};
use crate::utils;

pub fn rock_paper_scissors() -> Option<(i32, i32)> {
    let file = match utils::read_file("day_2_input.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Couldn't open file {}", e);
            return None;
        }
    };

    let reader = BufReader::new(file);
    let mut score_part1 = 0;
    let mut score_part2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(' ').collect();
        let op_move = moves[0];
        let my_move = moves[1];

        // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.

        // Not my best implementation :yikes:
        match op_move {
            "A" => { // Rock 1
                match my_move {
                    "X" => {
                        score_part1 += 1 + 3; // Rock
                        score_part2 += 3 + 0; // Lose
                    },
                    "Y" => {
                        score_part1 += 2 + 6; // Paper
                        score_part2 += 1 + 3; // Draw
                    },
                    "Z" => {
                        score_part1 += 3 + 0; // Scissors
                        score_part2 += 2 + 6; // Win
                    },
                    _ => continue
                }
            },
            "B" => { // Paper 2
                match my_move {
                    "X" => {
                        score_part1 += 1 + 0; // Rock
                        score_part2 += 1 + 0;
                    },
                    "Y" => {
                        score_part1 += 2 + 3; // Paper
                        score_part2 += 2 + 3;
                    },
                    "Z" => {
                        score_part1 += 3 + 6; // Scissors
                        score_part2 += 3 + 6;
                    },
                    _ => continue
                }
            },
            "C" => { // Scissors 3
                match my_move {
                    "X" => {
                        score_part1 += 1 + 6; // Rock
                        score_part2 += 2 + 0;
                    },
                    "Y" => {
                        score_part1 += 2 + 0; // Paper
                        score_part2 += 3 + 3;
                    },
                    "Z" => {
                        score_part1 += 3 + 3; // Scissors
                        score_part2 += 1 + 6;
                    },
                    _ => continue
                }
            },
            _ => continue,
        }
    }

    println!("Total score {}, {}", score_part1, score_part2);

    Some((score_part1, score_part2))
}
