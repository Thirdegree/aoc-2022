use std::{collections::HashSet, thread::sleep, time::Duration};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
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
            for x_move in if x == other.x { 0..1 } else { -1..2 } {
                for y_move in if y == other.y { 0..1 } else { -1..2 } {
                    let test = Self {
                        x: x + x_move,
                        y: y + y_move,
                    };
                    let test_dist = test.dist(other);
                    let best_move_dist = best_move.dist(other);
                    if test_dist < best_move_dist {
                        best_move = test;
                    }
                    if test_dist == best_move_dist && (test.x == other.x || test.y == other.y) {
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
    knots: Vec<Coord>,
    visited: Vec<Coord>,
}

impl From<&str> for Rope {
    fn from(input: &str) -> Self {
        Self {
            steps: input.lines().map(|l| l.into()).collect(),
            head: Coord { x: 0, y: 0 },
            knots: vec![Coord { x: 0, y: 0 }; 9],
            visited: vec![Coord { x: 0, y: 0 }],
        }
    }
}

impl Rope {
    fn run(&mut self, steps: &mut Vec<Step>) {
        for step in steps.iter_mut() {
            self.step(&step);
            self.draw();
        }
    }
    fn draw(&self) {
        let mut visited: HashSet<_> = self.visited.iter().collect();
        visited.insert(&self.head);
        visited.extend(&self.knots);
        let min_x = visited.iter().map(|c| c.x).min().unwrap();
        let max_x = visited.iter().map(|c| c.x).max().unwrap();
        let min_y = visited.iter().map(|c| c.y).min().unwrap();
        let max_y = visited.iter().map(|c| c.y).max().unwrap();
        let mut out: Vec<String> = Vec::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = Coord { x, y };
                if c == self.head {
                    out.push("H".into());
                } else if let Some(f) =
                    self.knots
                        .iter()
                        .enumerate()
                        .find_map(|(i, e)| if e == &c { Some(i) } else { None })
                {
                    out.push((f + 1).to_string());
                } else if visited.contains(&c) {
                    out.push("#".into());
                } else {
                    out.push(".".into());
                }
            }
            out.push("\n".into());
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let out = out[..(65*252).min(out.len())].join("");
        print!("{}", out);
        sleep(Duration::from_millis(20));
    }
    fn step(&mut self, step: &Step) {
        match step {
            Step::Up(count) => {
                for _ in 0..*count {
                    self.head = Coord {
                        x: self.head.x,
                        y: self.head.y - 1,
                    };
                    let new_knot = self.knots[0].move_closer(&self.head);
                    self.knots[0] = new_knot;
                    //self.visited.push(self.knots[0]);
                    for elem in 1..self.knots.len() {
                        let new_knot = self.knots[elem].move_closer(&self.knots[elem - 1]);
                        self.knots[elem] = new_knot;
                    }
                    self.visited.push(self.knots[8]);
                    // self.draw();
                }
            }
            Step::Down(count) => {
                for _ in 0..*count {
                    self.head = Coord {
                        x: self.head.x,
                        y: self.head.y + 1,
                    };
                    let new_knot = self.knots[0].move_closer(&self.head);
                    self.knots[0] = new_knot;
                    //self.visited.push(self.knots[0]);
                    for elem in 1..self.knots.len() {
                        let new_knot = self.knots[elem].move_closer(&self.knots[elem - 1]);
                        self.knots[elem] = new_knot;
                        //self.visited.push(self.knots[elem]);
                    }
                    self.visited.push(self.knots[8]);
                    // self.draw();
                }
            }
            Step::Left(count) => {
                for _ in 0..*count {
                    self.head = Coord {
                        x: self.head.x - 1,
                        y: self.head.y,
                    };
                    let new_knot = self.knots[0].move_closer(&self.head);
                    self.knots[0] = new_knot;
                    //self.visited.push(self.knots[0]);
                    for elem in 1..self.knots.len() {
                        let new_knot = self.knots[elem].move_closer(&self.knots[elem - 1]);
                        self.knots[elem] = new_knot;
                        //self.visited.push(self.knots[elem]);
                    }
                    self.visited.push(self.knots[8]);
                    // self.draw();
                }
            }
            Step::Right(count) => {
                for _ in 0..*count {
                    self.head = Coord {
                        x: self.head.x + 1,
                        y: self.head.y,
                    };
                    let new_knot = self.knots[0].move_closer(&self.head);
                    self.knots[0] = new_knot;
                    //self.visited.push(self.knots[0]);
                    for elem in 1..self.knots.len() {
                        let new_knot = self.knots[elem].move_closer(&self.knots[elem - 1]);
                        self.knots[elem] = new_knot;
                        //self.visited.push(self.knots[elem]);
                    }
                    self.visited.push(self.knots[8]);
                    // self.draw();
                }
            }
        }
    }
}

pub fn main() {
    let mut rope: Rope = include_str!("data/day9.txt").into();
    let mut steps = Vec::new();
    steps.clone_from(&rope.steps);
    rope.run(&mut steps);
    rope.draw();
    let uniq: HashSet<_> = rope.visited.iter().collect();
    println!("uniq: {}", uniq.len());
}

#[cfg(test)]
mod test {
    use super::*;
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
