use std::collections::{VecDeque, HashSet};

pub fn main() {
    let input = include_str!("data/day6.txt");
    let mut last_four = VecDeque::new();
    for (i, char) in input.chars().enumerate() {
        if last_four.len() != 14 {
            last_four.push_back(char);
            continue;
        }
        last_four.pop_front();
        last_four.push_back(char);
        let collected = last_four.iter().collect::<HashSet<&char>>();
        if collected.len() == 14 {
            println!("Index: {}", i+1);
            break;
        }
    }
}
