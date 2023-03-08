use std::collections::HashSet;

pub fn turning_rouble(input: &str, after: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..input.len() {
        let recent: HashSet<_> = chars[i..i+after].iter().collect();
        if recent.len() == after {
            return i + after;
        }
    }
    return 0;
}