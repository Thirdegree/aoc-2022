use std::ops::Range;

#[derive(Debug)]
enum Step {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl From<&str> for Step {
    fn from(line: &str) -> Self {
        match line.split(" ").collect::<Vec<_>>()[..] {
            ["U", how_many] => Self::Up(how_many.parse().unwrap()),
            ["D", how_many] => Self::Down(how_many.parse().unwrap()),
            ["L", how_many] => Self::Left(how_many.parse().unwrap()),
            ["R", how_many] => Self::Right(how_many.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn dist(&self, other: &Self) -> usize {
        (self.x - other.x).abs().max((self.y - other.y).abs()) as usize
    }
    fn move_closer(self, other: &Self) -> Self {
        let dist = self.dist(other);
        if dist <= 1 {
            self
        } else {
            let x = self.x;
            let y = self.y;
            let mut best_move = self;
            for x_move in if x == other.x { 0..1 } else {-1..2} {
                for y_move in if y == other.y {0..1} else {-1..2}{
                    let test = Self {
                        x: x + x_move,
                        y: y + y_move,
                    };
                    dbg!(&test);
                    if dbg!(test.dist(other)) < dbg!(best_move.dist(other)) {
                        best_move = test;
                    }
                }
            }
            best_move
        }
    }
}

#[derive(Debug)]
struct Rope {
    steps: Vec<Step>,
    head: Coord,
    tail: Coord,
    visited: Vec<Coord>,
}

impl From<&str> for Rope {
    fn from(input: &str) -> Self {
        Self {
            steps: input.lines().map(|l| l.into()).collect(),
            head: Coord { x: 0, y: 0 },
            tail: Coord { x: 0, y: 0 },
            visited: vec![Coord { x: 0, y: 0 }],
        }
    }
}

impl Rope {
    fn step(&mut self, step: Step) {
        match step {
            Step::Up(count) => (),
            Step::Down(count) => (),
            Step::Left(count) => (),
            Step::Right(count) => (),
        }
    }
}

pub fn main() {
    let rope: Rope = include_str!("data/day9.txt").into();
    println!("{:?}", rope);
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! test_move_closer {
        (($end_x:literal, $end_y:literal)) => {
            
        };
    }
    #[test]
    fn test_move_closer() {
        assert_eq!(
            Coord { x: 1, y: 1 },
            Coord { x: 0, y: 0 }.move_closer(&Coord { x: 2, y: 2 })
        );
        assert_eq!(
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: 0 }.move_closer(&Coord { x: 0, y: 2 })
        );
        assert_eq!(
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 1 }.move_closer(&Coord { x: 2, y: 2 })
        );
    }
}
