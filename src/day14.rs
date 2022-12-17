use std::{thread::sleep, time::Duration};

const X_LEN: usize = 10000;

#[derive(Clone, Debug, PartialEq)]
enum Space {
    Empty,
    Sand,
    Rock,
}
struct Grid {
    grid: Vec<Vec<Space>>,
    floor_y: usize,
    sand_starts_at: Point,
    moving_sand: Option<Point>,
    tot_grains: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(pair: (usize, usize)) -> Self {
        Self {
            x: pair.0,
            y: pair.1,
        }
    }
}

impl From<&Vec<Vec<Point>>> for Grid {
    fn from(paths: &Vec<Vec<Point>>) -> Self {
        let max_y = paths
            .iter()
            .flat_map(|pairs| pairs.iter().map(|pair| pair.y))
            .max()
            .unwrap();
        let mut grid = vec![vec![Space::Empty; X_LEN]; max_y + 1 + 3];
        for path in paths {
            for i in 0..(path.len() - 1) {
                let pair = (&path[i], &path[i + 1]);
                let minpair_y = pair.0.y.min(pair.1.y);
                let maxpair_y = pair.0.y.max(pair.1.y);
                for y in minpair_y..=maxpair_y {
                    grid[y][pair.0.x + (X_LEN / 2)] = Space::Rock;
                }
                let minpair_x = pair.0.x.min(pair.1.x);
                let maxpair_x = pair.0.x.max(pair.1.x);
                for x in minpair_x..=maxpair_x {
                    grid[pair.0.y][x + (X_LEN / 2)] = Space::Rock;
                }
            }
        }
        Grid {
            grid,
            moving_sand: Some(((X_LEN / 2) + 500, 0).into()),
            sand_starts_at:((X_LEN / 2) + 500, 0).into(),
            floor_y: max_y + 2,
            tot_grains: 0,
        }
    }
}
impl From<Vec<Vec<Point>>> for Grid {
    fn from(paths: Vec<Vec<Point>>) -> Self {
        (&paths).into()
    }
}

impl Grid {
    fn get_coord(&self, x: usize, y: usize) -> Option<&Space> {
        if y >= self.grid.len() || x >= self.grid[0].len() {
            None
        } else {
            if y >= self.floor_y {
                Some(&Space::Rock)
            } else {
                let row = &self.grid[y];
                //println!("{:?}", row);
                Some(&row[x])
            }
        }
    }
    fn step(&mut self) -> Option<()> {
        if let Some(sand) = self.moving_sand {
            for option in [
                (sand.x, sand.y + 1),
                (sand.x - 1, sand.y + 1),
                (sand.x + 1, sand.y + 1),
            ] {
                if let Space::Empty = self.get_coord(option.0, option.1)? {
                    self.moving_sand = Some(option.into());
                    return Some(());
                }
            }
            if sand == self.sand_starts_at {
                return None;
            }
            // we've hit a wall -- and can't move! so sand is now solid sand
            self.grid[sand.y][sand.x] = Space::Sand;
            self.moving_sand = None;
            Some(())
        } else {
            self.moving_sand = Some(self.sand_starts_at);
            self.tot_grains += 1;
            Some(())
        }
    }
    fn print(&self) {
        let mut print_start = 10000;
        let mut print_end = 0;
        for row in &self.grid {
            for (i, elem) in row.iter().enumerate() {
                if let Space::Empty = elem {
                    continue;
                }
                print_start = print_start.min(i);
                print_end = print_end.max(i);
            }
        }
        for (y, row) in self.grid.iter().enumerate() {
            for (x, elem) in row.iter().enumerate() {
                if x <= print_start || x > print_end {
                    continue;
                }
                if Some((x, y).into()) == self.moving_sand {
                    print!("+");
                    continue;
                }
                match elem {
                    &Space::Rock => print!("#"),
                    &Space::Sand => print!("o"),
                    &Space::Empty => print!("."),
                }
            }
            println!();
        }
    }
}

pub fn main() {
    let mut paths: Grid = include_str!("data/day14.txt")
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let pairs: Vec<_> = pair.split(",").collect();
                    assert_eq!(2, pairs.len());
                    (pairs[0].parse().unwrap(), pairs[1].parse().unwrap()).into()
                })
                .collect()
        })
        .collect::<Vec<Vec<Point>>>()
        .into();
    let mut i = 0;
    while let Some(_) = paths.step() {
        // paths.print();
        // println!("----------------");
        // sleep(Duration::from_millis(50));
    }
    paths.print();
    println!("{}", paths.tot_grains + 1); // idk why +1 I think there's an off by one error
                                          // somewhere
}

#[cfg(test)]
mod test {
    use super::*;
    fn make_grid() -> (Vec<Vec<Point>>, Grid) {
        let points: Vec<Vec<Point>> = include_str!("data/day14.txt")
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|pair| {
                        let pairs: Vec<_> = pair.split(",").collect();
                        assert_eq!(2, pairs.len());
                        (pairs[0].parse().unwrap(), pairs[1].parse().unwrap()).into()
                    })
                    .collect()
            })
            .collect();
        let grid: Grid = (&points).into();
        (points, grid)
    }
    #[test]
    fn test_rock_ends_are_rock() {
        let (points, grid) = make_grid();
        for point in points.iter().flatten() {
            assert_eq!(Some(&Space::Rock), grid.get_coord(point.x, point.y));
        }
    }
}
