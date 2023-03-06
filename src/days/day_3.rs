use std::collections::HashSet;
use std::io::{BufRead};
use crate::utils::read_file;

pub fn rucksack_reorganization() -> Option<i32> {
    let reader = match read_file("3") {
        Some(reader) => reader,
        None => return None
    };

    let mut total_priorities = 0;

    for line in reader.lines() {
        let line = line.ok()?;

        let (first, last) = line.as_bytes().split_at(line.len() / 2);
        let f: HashSet<_> = first.into_iter().collect();
        let l: HashSet<_> = last.into_iter().collect();

        let item = **f.intersection(&l).next()? as char;
        total_priorities += add_priority(item);
    }

    Some(total_priorities)
}

pub fn rucksack_reorganization_badges() -> Option<i32> {
    let reader = match read_file("3") {
        Some(reader) => reader,
        None => return None
    };

    let mut total_badges = 0;

    let all_lines = reader.lines().collect::<Vec<_>>();

    for lines in all_lines.chunks(3) {
        let group: Vec<HashSet<_>> = lines
            .into_iter()
            .map(|line| {
                match line {
                    Ok(l) => l.as_bytes().into_iter().collect(),
                    Err(e) => {
                        eprintln!("{}", e);
                        "".as_bytes().into_iter().collect()
                    }
                }
            })
            .collect::<Vec<_>>();

        let badge = **(
            group[0].intersection(&group[1])
                .map(|e| *e)
                .collect::<HashSet<&u8>>()
        ).intersection(&group[2]).next()? as char;

        total_badges += add_priority(badge);
    }

    Some(total_badges)
}

fn add_priority(item: char) -> i32 {
    return if item.is_ascii_uppercase() { (item as i32) - 38 } else { (item as i32) - 96 };
}