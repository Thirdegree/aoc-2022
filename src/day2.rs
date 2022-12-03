#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err("Oh no failed for {value}"),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {}

impl TryFrom<char> for Outcome {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Oh no failed for {value}"),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Round {
    opponent_move: Move,
    desired_outcome: Outcome,
}

impl TryFrom<&str> for Round {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            opponent_move: Move::try_from(value.chars().nth(0).unwrap())?,
            desired_outcome: Outcome::try_from(value.chars().nth(2).unwrap())?,
        })
    }
}

pub fn main() {
    let rounds: Vec<Round> = include_str!("data/day2.txt")
        .lines()
        .filter_map(|item| Round::try_from(item).ok())
        .collect();
    println!("here we go");
    let mut tot_score = 0;
    for round in rounds.iter() {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        {
            tot_score += match round {
                // Wins
                Round { opponent_move: Move::Rock, desired_outcome: Outcome::Win, } => Move::Paper.score() + 6,
                Round { opponent_move: Move::Paper, desired_outcome: Outcome::Win, } => Move::Scissors.score() + 6,
                Round { opponent_move: Move::Scissors, desired_outcome: Outcome::Win, } => Move::Rock.score() + 6,
                // Tie is always just the same move
                Round { opponent_move, desired_outcome: Outcome::Draw, } => opponent_move.score() + 3,
                // Losses
                Round { opponent_move: Move::Rock, desired_outcome: Outcome::Lose, } => Move::Scissors.score() + 0,
                Round { opponent_move: Move::Paper, desired_outcome: Outcome::Lose, } => Move::Rock.score() + 0,
                Round { opponent_move: Move::Scissors, desired_outcome: Outcome::Lose, } => Move::Paper.score() + 0,
            };
        }
    }
    println!("Score: {}", tot_score);
}
