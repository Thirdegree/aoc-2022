use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Write;

#[derive(Eq, PartialEq)]
enum MapItem {
    Best,
    Square(usize),
}

impl Debug for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Best => f.write_char('S'),
            Self::Square(v) => f.write_char((*v as u8 + 50) as char),
        }
    }
}

#[derive(Debug)]
struct HeightMap {
    grid: Vec<Vec<MapItem>>,
}

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        HeightMap {
            grid: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            'S' => MapItem::Square(('a' as usize) - 50),
                            'E' => MapItem::Best,
                            place => MapItem::Square((place as usize) - 50),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl HeightMap {
    fn find_best_best(&self) {
        let best = self.grid.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().filter_map(|(x, item)| {
                if let MapItem::Square(s) = item {
                    if s == &(('a' as usize) - 50) {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }).collect::<Vec<(i32, i32)>>()
        }).flatten().filter_map(|start| self.find_best(start)).min();
        println!("{:?}", best);
    }

    fn find_best(&self, start: (i32, i32)) -> Option<usize> {
        println!("Trying with start {:?}", start);
        let mut que = VecDeque::new();
        let mut adjacency_list: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::default();
        let mut explored: HashMap<(i32, i32), (i32, i32)> = HashMap::default();
        let mut seen: HashSet<(i32, i32)> = HashSet::default();
        seen.insert(start);

        que.push_back((None::<(i32, i32)>, start));
        let mut best = None::<(i32, i32)>;
        while let Some((prev, elem)) = que.pop_front() {
            if best.is_some() {
                continue;
            }
            if let Some(prev) = prev {
                if !adjacency_list.contains_key(&prev) {
                    adjacency_list.insert(prev, Vec::new());
                }
                adjacency_list.get_mut(&prev).unwrap().push(elem);
            }
            let cur_pos = match &self.grid[elem.1 as usize][elem.0 as usize] {
                MapItem::Square(height) => height,
                MapItem::Best => &(('z' as usize) - 50),
            };
            'exploring: for y in -1..=1 {
                let explore_y = elem.1 + y;
                if explore_y < 0 || explore_y >= self.grid.len() as i32 {
                    continue;
                }
                for x in -1..=1 {
                    let explore_x = elem.0 + x;
                    if explore_x < 0 || explore_x >= self.grid[explore_y as usize].len() as i32 {
                        continue;
                    }
                    if (explore_x, explore_y) == elem {
                        continue;
                    }
                    if y != 0 && x != 0 {
                        continue;
                    }
                    if seen.contains(&(explore_x, explore_y)) {
                        continue;
                    }
                    match &self.grid[explore_y as usize][explore_x as usize] {
                        MapItem::Best => {
                            if (('z' as usize) - 50) <= (cur_pos + 1) {
                                que.push_back((Some(elem), (explore_x, explore_y)));
                                explored.insert((explore_x, explore_y), elem);
                                seen.insert((explore_x, explore_y));
                                best = Some((explore_x, explore_y));
                                break 'exploring;
                            }
                        }
                        MapItem::Square(height) => {
                            if height <= &(cur_pos + 1) {
                                que.push_back((Some(elem), (explore_x, explore_y)));
                                explored.insert((explore_x, explore_y), elem);
                                seen.insert((explore_x, explore_y));
                            }
                        }
                    }
                }
            }
        }
        //println!("{:?}", best);
        //let mut print_grid = vec![vec!["."; self.grid[0].len()]; self.grid.len()];

        let mut cur = best?;
        //print_grid[cur.1 as usize][cur.0 as usize] = "E";
        let mut path = Vec::new();
        path.push(cur);
        let mut cycle_detect = HashSet::new();
        while let Some(prev) = explored.get(&(cur)) {
            path.push(*prev);
            // println!(
            //     "{:?} {:?} {:?}",
            //     prev,
            //     cur,
            //     (prev.0 - cur.0, prev.1 - cur.1)
            // );
            if cycle_detect.contains(prev) {
                panic!("Cycle! {:?} {:?}", cur, prev);
            }
            cycle_detect.insert(prev);
            // match (prev.0 - cur.0, prev.1 - cur.1) {
            //     (0, -1) => print_grid[prev.1 as usize][prev.0 as usize] = "v", // down
            //     (0, 1) => print_grid[prev.1 as usize][prev.0 as usize] = "^",  // up
            //     (1, 0) => print_grid[prev.1 as usize][prev.0 as usize] = "<",  // right
            //     (-1, 0) => print_grid[prev.1 as usize][prev.0 as usize] = ">", // left
            //     _ => unreachable!(),
            // }
            cur = *prev;
        }

        //for row in print_grid {
        //    println!("{}", row.join(""));
        //}
        Some(path.len() - 1)
    }
}

pub fn main() {
    let map: HeightMap = include_str!("data/day12.txt").into();
    println!("{:?}", map);
    map.find_best_best();
}

#[cfg(test)]
mod test {
    #[test]
    fn test_level_conversion() {}
}
