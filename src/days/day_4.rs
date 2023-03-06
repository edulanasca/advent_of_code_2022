use std::collections::HashSet;
use std::io::BufRead;
use crate::utils::read_file;

pub fn camp_cleanup() -> Option<(i32, i32)> {
    let reader = match read_file("4") {
        Some(reader) => reader,
        None => return None
    };

    let mut n_overlap = 0;
    let mut n_full_overlap = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let pairs: Vec<&str> = line.split(",").collect();
        let g1: HashSet<i32> = generate_array(pairs[0]).into_iter().collect();
        let g2: HashSet<i32> = generate_array(pairs[1]).into_iter().collect();

        if g1.is_superset(&g2) || g1.is_subset(&g2) { n_overlap += 1 }
        if !g1.is_disjoint(&g2) { n_full_overlap += 1 }
    }

    Some((n_overlap, n_full_overlap))
}

fn generate_array(from: &str) -> Vec<i32> {
    let numbers: Vec<&str> = from.split("-").collect();
    let n1 = numbers[0].parse().unwrap();
    let n2 = numbers[1].parse().unwrap();

    let mut v: Vec<i32> = (n1..n2).collect();
    v.push(n2);

    if n1 == n2 { vec![n1] } else { v }
}