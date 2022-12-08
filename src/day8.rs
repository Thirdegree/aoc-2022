#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u32>>,
}

impl Grid {
    fn visible(&self, x: usize, y: usize) -> bool {
        let row = &self.grid[y];
        let col: Vec<_> = self.grid.iter().map(|line| line[x]).collect();
        row[..x].iter().all(|&elem| elem < row[x])
            || row[x + 1..].iter().all(|&elem| elem < row[x])
            || col[..y].iter().all(|&elem| elem < col[y])
            || col[y + 1..].iter().all(|&elem| elem < col[y])
    }
    fn count_visible(&self) -> usize {
        (0..self.grid.len())
            .map(|y| {
                (0..self.grid[y].len())
                    .filter(|x| self.visible(*x, y))
                    .count()
            })
            .sum()
    }
    fn senic_score(&self, x: usize, y: usize) -> usize {
        let row = &self.grid[y];
        let col: Vec<_> = self.grid.iter().map(|line| line[x]).collect();
        let consider = self.grid[y][x];
        let mut score = 1;
        // note: do not like
        // It's important that we do 1..x and x+1..row.len()-1
        // I choose not to elaborate why.
        score *= &row[1..x]
            .iter()
            .rev()
            .take_while(|&&t| t < consider)
            .count()
            + 1;
        score *= &row[x + 1..row.len() - 1]
            .iter()
            .take_while(|&&t| t < consider)
            .count()
            + 1;
        score *= &col[1..y]
            .iter()
            .rev()
            .take_while(|&&t| t < consider)
            .count()
            + 1;
        score *= &col[y + 1..col.len() - 1]
            .iter()
            .take_while(|&&t| t < consider)
            .count()
            + 1;
        score
    }
    fn most_senic_score(&self) -> usize {
        (1..self.grid.len() - 1)
            .map(|y| {
                (1..self.grid[y].len() - 1)
                    .map(|x| self.senic_score(x, y))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

pub fn main() {
    let grid = Grid {
        grid: include_str!("data/day8.txt")
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect(),
    };
    println!("Visible: {}", grid.count_visible());
    println!("Most Senic: {}", grid.most_senic_score());
}

#[cfg(test)]
mod test {
    use super::Grid;

    const GRID: &str = "30373
25512
65332
33549
35390";
    fn parse() -> Grid {
        Grid {
            grid: GRID
                .lines()
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }

    #[test]
    fn test_visible_inner() {
        let grid = parse();
        assert!(grid.visible(1, 1));
        assert!(grid.visible(2, 1));
        assert!(grid.visible(4, 1));
        assert!(grid.visible(1, 2));
        assert!(grid.visible(3, 2));
        assert!(grid.visible(2, 3));
        // corners
        assert!(grid.visible(0, 0));
        assert!(grid.visible(4, 4));
        assert!(grid.visible(0, 4));
        assert!(grid.visible(4, 0));
    }

    #[test]
    fn test_invisible() {
        let grid = parse();
        assert!(!grid.visible(3, 1));
        assert!(!grid.visible(2, 2));
        assert!(!grid.visible(1, 3));
        assert!(!grid.visible(3, 3));
    }

    #[test]
    fn test_senic_score() {
        let grid = parse();
        assert_eq!(4, grid.senic_score(2, 1));
    }

    #[test]
    fn test_most_senic_score() {
        let grid = parse();
        assert_eq!(8, grid.most_senic_score());
    }
}
