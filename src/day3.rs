use std::collections::HashSet;

#[derive(Debug)]
pub struct Rucksack {
    front: HashSet<char>,
    back: HashSet<char>,
}

impl Rucksack {
    pub fn overlaps(&self) -> HashSet<&char> {
        self.front.intersection(&self.back).collect()
    }
    fn items(&self) -> HashSet<&char> {
        self.front.union(&self.back).collect()
    }
}

impl TryFrom<&str> for Rucksack {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() % 2 != 0 {
            return Err("Invalid length for input -- must be even");
        }
        Ok(Rucksack {
            front: value[..value.len() / 2].chars().collect(),
            back: value[value.len() / 2..].chars().collect(),
        })
    }
}

trait Prio {
    fn prio(&self) -> u32;
}

impl Prio for char {
    fn prio(&self) -> u32 {
        let ord = Into::<u32>::into(*self);
        match self {
            'a'..='z' => ord - 96, // 1-26
            'A'..='Z' => ord - 64 + 26, // 27-52
            _ => unreachable!(),
        }
    }
}

pub fn main() {
    let binding = include_str!("data/day3.txt")
        .lines()
        .filter_map(|item| Rucksack::try_from(item).ok())
        .collect::<Vec<_>>();
    let sacks: Vec<Vec<_>> = binding.chunks(3).map(|c| c.iter().collect()).collect();
    let mut prio_sum = 0;
    let inital_sack: HashSet<char> = ('a'..='z').chain('A'..='Z').collect();
    for sack in sacks {
        let badge = sack.iter().fold(inital_sack.clone(), |acc, r| {
            acc.intersection(&r.items().iter().map(|c| **c).collect())
                .map(|c| *c)
                .collect()
        });
        prio_sum += badge.iter().map(|c| c.prio()).sum::<u32>();
    }
    println!("Total Prio: {}", prio_sum);
}
