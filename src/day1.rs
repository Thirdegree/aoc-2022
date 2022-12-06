use std::num::ParseIntError;
use std::str::Lines;

trait Elves<'a> {
    fn elves(&mut self) -> Result<Vec<Vec<u32>>, ParseIntError>
    where
        Self: Iterator<Item = &'a str>,
    {
        let mut acc = vec![];
        let mut cur_elf = vec![];
        for line in self {
            if line.is_empty() {
                acc.push(cur_elf);
                cur_elf = vec![];
                continue;
            }
            cur_elf.push(line.parse()?);
        }
        Ok(acc)
    }
}

impl<'a> Elves<'a> for Lines<'a> {}

pub fn main() {
    let binding = include_str!("data/day1.txt").lines().elves().unwrap();
    let mut elves = binding.iter().collect::<Vec<_>>();
    elves.sort_by_key(|v| v.iter().sum::<u32>());
    println!(
        "Calories: {:?}",
        elves
            .iter()
            .rev()
            .take(3)
            .map(|v| v.iter().sum::<u32>())
            .sum::<u32>()
    );
}
