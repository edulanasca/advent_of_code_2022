use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use crate::utils::read_file;

const CRATES: fn(Vec<Vec<char>>) -> Option<String> = |stacks|
    Some(stacks
        .iter()
        .map(|vec| if !vec.is_empty() { (*vec).first().unwrap().to_string() } else { String::new() })
        .collect::<Vec<String>>()
        .join(""));

const PROCESS_COMMAND: fn(String) -> Vec<usize> = |line|
    line.split(" ")
        .map(|e| e.parse::<usize>().ok())
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect();

fn supply_stacks() -> Option<(Lines<BufReader<File>>, Vec<Vec<char>>)> {
    let reader = match read_file("5") {
        Some(reader) => reader,
        None => return None
    };

    let mut lines = reader.lines();
    let mut stacks: Vec<Vec<char>> = vec![];

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();

        if chars[1].is_numeric() { // Detects the end of the stacks
            lines.next(); // Skips empty line
            break;
        }

        let mut counter = 1;
        let mut i: usize = 0;

        while counter < chars.len() {
            let c = chars[counter];

            match stacks.get(i) {
                Some(_) => if c.is_alphabetic() { stacks[i].push(c) },
                None => if c.is_alphabetic() { stacks.push(vec![c]) } else { stacks.push(vec![]) }
            }

            i += 1;
            counter += 4;
        }
    }

    Some((lines, stacks))
}

pub fn crate_mover_9000() -> Option<String> {
    let stacks = match supply_stacks() {
        Some(lines) => lines,
        None => return None
    };

    let mut lines = stacks.0;
    let mut stacks = stacks.1;

    // Start processing commands
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let ins = PROCESS_COMMAND(line);

        let from = stacks[ins[1] - 1].clone();
        let slice = from.split_at(ins[0]);
        slice.0.iter().for_each(|e| stacks[ins[2] - 1].insert(0, *e));
        stacks[ins[1] - 1] = Vec::from(slice.1);
    }

    CRATES(stacks)
}

pub fn crate_mover_9001() -> Option<String> {
    let stacks = match supply_stacks() {
        Some(lines) => lines,
        None => return None
    };

    let mut lines = stacks.0;
    let mut stacks = stacks.1;

    // Start processing commands
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let ins = PROCESS_COMMAND(line);

        let from = stacks[ins[1] - 1].clone();

        let slice = from.split_at(ins[0]);
        let mut _move = Vec::from(slice.0);

        stacks[ins[2] - 1].iter().for_each(|e| _move.push(*e));

        stacks[ins[2] - 1] = _move;
        stacks[ins[1] - 1] = Vec::from(slice.1);
    }

    CRATES(stacks)
}