use nom::{
    character::complete::{char, one_of},
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::{
    collections::HashSet,
    ops::RangeInclusive,
};

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (input, (start_str, _, end_str)) = tuple((
        many1(one_of("1234567890")),
        char('-'),
        many1(one_of("1234567890")),
    ))(input)?;
    Ok((
        input,
        RangeInclusive::new(
            start_str.iter().collect::<String>().parse().unwrap(),
            end_str.iter().collect::<String>().parse().unwrap(),
        ),
    ))
}

fn parse_line(input: &str) -> IResult<&str, (RangeInclusive<usize>, RangeInclusive<usize>)> {
    let (input, (range1, _, rang2)) = tuple((parse_range, char(','), parse_range))(input)?;
    Ok((input, (range1, rang2)))
}

pub fn main() {
    let items: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = include_str!("data/day4.txt")
        .lines()
        .map(|l| parse_line(l).unwrap().1)
        .collect();
    let mut found = 0;
    for (first, second) in items {
        let first_items = first.collect::<HashSet<usize>>();
        let second_items = second.collect::<HashSet<usize>>();
        let res: Vec<&usize> = first_items.intersection(&second_items).collect();
        if !res.is_empty() {
            found += 1;
        }
    }
    println!("Overlaps: {}", found);
}
