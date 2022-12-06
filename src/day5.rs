use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    combinator::opt,
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(PartialEq, Eq, Debug)]
pub enum CratePresent {
    Present(Crate),
    Absent,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Crate {
    ident: String,
}

impl Crate {
    pub fn parse(input: &str) -> IResult<&str, CratePresent> {
        let (input, ident) = alt((delimited(tag("["), alpha1, tag("]")), tag("   ")))(input)?;
        let _crate = if ident == "   " {
            CratePresent::Absent
        } else {
            CratePresent::Present(Crate {
                ident: ident.to_string(),
            })
        };
        Ok((input, _crate))
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Move {
    how_many: usize,
    from: usize,
    to: usize,
}

pub fn parse_int<T: FromStr>(input: &str) -> IResult<&str, T> {
    let (input, found) = digit1(input)?;
    match found.parse() {
        Ok(val) => Ok((input, val)),
        Err(_) => todo!(),
    }
}

impl Move {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (_, move_v, _, from, _, to)) = tuple((
            tag("move "),
            parse_int,
            tag(" from "),
            parse_int,
            tag(" to "),
            parse_int,
        ))(input)?;
        Ok((
            input,
            Move {
                how_many: move_v,
                from,
                to,
            },
        ))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct CraneGame {
    pub stacks: Vec<Vec<Crate>>,
    pub moves: Vec<Move>,
}

impl CraneGame {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, stacks) = Self::parse_crates(input)?;
        let (input, moves) = preceded(
            tuple((many1(alt((digit1, space1))), tag("\n\n"))),
            many1(terminated(Move::parse, opt(newline))),
        )(input)?;
        Ok((input, CraneGame { stacks, moves }))
    }

    pub fn parse_crates(input: &str) -> IResult<&str, Vec<Vec<Crate>>> {
        let (input, crates_by_row) = many1(terminated(
            many1(terminated(Crate::parse, opt(tag(" ")))),
            newline,
        ))(input)?;
        let mut stacks = Vec::new();
        for _ in 0..crates_by_row[0].len() {
            stacks.push(Vec::new());
        }
        for row in crates_by_row {
            for (stack, _crate) in row.iter().enumerate() {
                if let CratePresent::Present(c) = _crate {
                    stacks[stack].push(c.to_owned());
                }
            }
        }
        for stack in &mut stacks {
            stack.reverse();
        }
        Ok((input, stacks))
    }

    fn solve(&mut self) -> String {
        for _move in &self.moves {
            let mut tmp = Vec::new();
            for _ in 0.._move.how_many {
                let elem = self.stacks[_move.from - 1].pop().unwrap();
                tmp.push(elem)
            }
            tmp.reverse();
            self.stacks[_move.to - 1].extend(tmp);
        }
        self.stacks
            .iter()
            .map(|s| s[s.len() - 1].ident.clone())
            .collect()
    }
}

pub fn main() {
    let input = include_str!("data/day5.txt");
    dbg!(CraneGame::parse(input).unwrap().1.solve());
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_move_parse() {
        assert_eq!(
            Move::parse("move 1 from 2 to 1").unwrap().1,
            Move {
                how_many: 1,
                from: 2,
                to: 1
            }
        );
    }

    #[test]
    fn test_crate_parse() {
        assert_eq!(
            Crate::parse("[A]").unwrap().1,
            CratePresent::Present(Crate {
                ident: "A".to_string()
            })
        );
    }

    #[test]
    fn test_crate_parse_empty() {
        assert_eq!(Crate::parse("   ").unwrap().1, CratePresent::Absent);
    }

    #[test]
    fn test_parse_crates() {
        let input = concat!("    [D]    \n", "[N] [C]    \n", "[Z] [M] [P]\n");

        assert_eq!(
            CraneGame::parse_crates(input).unwrap().1,
            vec![
                vec![
                    Crate {
                        ident: "Z".to_string()
                    },
                    Crate {
                        ident: "N".to_string()
                    }
                ],
                vec![
                    Crate {
                        ident: "M".to_string()
                    },
                    Crate {
                        ident: "C".to_string()
                    },
                    Crate {
                        ident: "D".to_string()
                    }
                ],
                vec![Crate {
                    ident: "P".to_string()
                }],
            ]
        )
    }

    #[test]
    fn test_parse_cranegame() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let game = CraneGame::parse(input).unwrap().1;
        assert_eq!(
            game,
            CraneGame {
                stacks: vec![
                    vec![
                        Crate {
                            ident: "Z".to_string()
                        },
                        Crate {
                            ident: "N".to_string()
                        }
                    ],
                    vec![
                        Crate {
                            ident: "M".to_string()
                        },
                        Crate {
                            ident: "C".to_string()
                        },
                        Crate {
                            ident: "D".to_string()
                        }
                    ],
                    vec![Crate {
                        ident: "P".to_string()
                    }],
                ],
                moves: vec![
                    Move {
                        how_many: 1,
                        from: 2,
                        to: 1
                    },
                    Move {
                        how_many: 3,
                        from: 1,
                        to: 3
                    },
                    Move {
                        how_many: 2,
                        from: 2,
                        to: 1
                    },
                    Move {
                        how_many: 1,
                        from: 1,
                        to: 2
                    },
                ]
            }
        )
    }
}
