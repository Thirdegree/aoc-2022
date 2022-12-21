use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

const X_LEN: usize = 7;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Rock {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square,
}

impl Rock {
    fn as_coords(&self, top_left: &Point) -> Vec<Point> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match self {
            &Rock::Horizontal => vec![
                Point {x: top_left.x, y: top_left.y},
                Point { x: top_left.x + 1, y: top_left.y, },
                Point { x: top_left.x + 2, y: top_left.y, },
                Point { x: top_left.x + 3, y: top_left.y, },
            ],
            &Rock::Plus => vec![
                Point { x: top_left.x + 1, y: top_left.y, },
                Point {x: top_left.x, y: top_left.y - 1},
                Point {x: top_left.x + 1, y: top_left.y - 1},
                Point {x: top_left.x + 2, y: top_left.y - 1},
                Point {x: top_left.x + 1, y: top_left.y - 2},
            ],
            &Rock::L => vec![
                Point{x: top_left.x + 2, y: top_left.y},
                Point{x: top_left.x + 2, y: top_left.y - 1},
                Point{x: top_left.x + 2, y: top_left.y - 2},
                Point{x: top_left.x + 1, y: top_left.y - 2},
                Point{x: top_left.x, y: top_left.y - 2},
            ],
            &Rock::Vertical => vec![
                Point {x: top_left.x, y: top_left.y},
                Point{x: top_left.x, y: top_left.y - 1},
                Point{x: top_left.x, y: top_left.y - 2},
                Point{x: top_left.x, y: top_left.y - 3},
            ],
            &Rock::Square => vec![
                Point {x: top_left.x, y: top_left.y},
                Point{x: top_left.x + 1, y: top_left.y},
                Point{x: top_left.x + 1, y: top_left.y - 1},
                Point{x: top_left.x, y: top_left.y - 1},
            ]
        }
    }
    fn next(&self) -> Self {
        match self {
            Self::Horizontal => Self::Plus,
            Self::Plus => Self::L,
            Self::L => Self::Vertical,
            Self::Vertical => Self::Square,
            Self::Square => Self::Horizontal,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Space {
    Empty,
    Rock,
}

struct Grid {
    grid: Vec<[Space; X_LEN]>,
    cur_top_left: Point,
    cur_rock: Rock,
    highest_rock_y: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut base = self.base_disp_grid();
        for point in self.cur_rock.as_coords(&self.cur_top_left) {
            (point, &base.len(), &base[0].len(), self.cur_top_left);
            base[point.y][point.x] = '@';
        }
        f.write_str(
            base.iter()
                .map(|col| col.iter().collect())
                .intersperse("\n".to_string())
                .collect::<String>()
                .as_str(),
        )
    }
}

impl Grid {
    fn new() -> Self {
        let highest_y = 0;
        Self {
            grid: Vec::default(),
            cur_top_left: Point { x: 2, y: 3 },
            cur_rock: Rock::Horizontal,
            highest_rock_y: highest_y,
        }
    }
    fn base_disp_grid(&self) -> Vec<Vec<char>> {
        dbg!(self.highest_rock_y, self.cur_top_left);
        let after = vec![
            vec!['.'; X_LEN];
            (self.highest_rock_y.max(self.cur_top_left.y) - self.grid.len()) + 1
        ];
        let mut base: Vec<Vec<char>> = self
            .grid
            .iter()
            .rev()
            .map(|col| {
                col.iter().map(|elem| match elem {
                    Space::Empty => '.',
                    Space::Rock => '#',
                }).collect()
            })
            .collect();
        base.extend(after);
        base
    }
    fn lock_in(&mut self, point: &Point) {
        let mut new_h_y = self.highest_rock_y;
        for space in self.cur_rock.as_coords(point) {
            self[space] = Space::Rock;
            new_h_y = new_h_y.max(space.y)
        }
        self.highest_rock_y = new_h_y;
    }
    fn run(&mut self) {
        let winddiriter = include_str!("data/day17.txt")
            .strip_suffix("\n")
            .unwrap()
            .chars()
            .cycle();
        let (left, right) = (0, X_LEN);
        for wind in winddiriter {
            println!("{}", self);
            println!("-------------------------");
            // move with the wind
            let poses = self.cur_rock.as_coords(&self.cur_top_left);
            match wind {
                '>' if !poses.iter().any(|&elem| (elem.x + 1) >= right) => self.cur_top_left.x += 1,
                '<' if !poses.iter().any(|&elem| (elem.x - 1) < left) => self.cur_top_left.x -= 1,
                '<' | '>' => {}
                _ => unreachable!(),
            }
            dbg!(self.cur_top_left);
            // falllllllllllllllllllllllllllllllllllllll
            let mut locked = false;
            if self.cur_top_left.y == 0 {
                let top_left = self.cur_top_left;
                self.lock_in(&top_left);
                locked = true;
            } else {
                for space in self.cur_rock.as_coords(&Point {
                    x: self.cur_top_left.x,
                    y: self.cur_top_left.y - 1,
                }) {
                    if let Space::Rock = &self[space] {
                        let top_left = self.cur_top_left;
                        self.lock_in(&top_left);
                        locked = true;
                        break;
                    }
                }
            }
            if !locked {
                self.cur_top_left = (Point {
                    x: self.cur_top_left.x,
                    y: self.cur_top_left.y - 1,
                });
            } else {
                self.cur_rock = (self.cur_rock.next());
                self.cur_top_left = Point {
                    x: 2,
                    y: self.highest_rock_y
                        + match self.cur_rock {
                            Rock::Vertical => 4,
                            Rock::L | Rock::Plus => 3,
                            Rock::Square => 2,
                            Rock::Horizontal => 1,
                        },
                };
            }
        }
    }
}

impl Index<Point> for Grid {
    type Output = Space;

    fn index(&self, index: Point) -> &Self::Output {
        // higher means TALLER, but we store rows the opposite. So we need to translate that.
        let highest_y = self.highest_rock_y.max(self.cur_top_left.y);
        if highest_y <= index.y {
            // if someone is asking higher than we know, it's definitely empty.
            &Space::Empty
        } else {
            // else, we want to figure out how high from the top it should be
            &self.grid[index.y][index.x]
        }
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let highest_y = self.highest_rock_y.max(self.cur_top_left.y);
        if highest_y <= index.y {
            self.grid
                .append(&mut vec![[Space::Empty; X_LEN]; index.y - highest_y + 1]);
        };
        &mut self.grid[index.y][index.x]
    }
}

pub fn main() {
    let rocks = vec![
        Rock::Horizontal,
        Rock::Plus,
        Rock::L,
        Rock::Vertical,
        Rock::Square,
    ];
    let mut rockiter = rocks.iter().cycle();
    let mut cur_top_left = Point { x: 0, y: 0 };
    let cur_rock = rockiter.next().unwrap();
    let mut grid = Grid::new();
    grid.run();
}
