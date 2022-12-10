use std::{path::PathBuf, collections::HashMap};

pub fn main() {
    let mut pwd = PathBuf::new();
    let mut map: HashMap<PathBuf, usize> = HashMap::new();

    include_str!("data/day7.txt")
        .lines()
        .for_each(|line| match line.split(" ").collect::<Vec<_>>()[..] {
            ["$", "cd", dir] => {pwd.push(dir); pwd = pwd.canonicalize().unwrap()},
            ["$", "ls"] => (),
            ["dir", _] => (),
            [size, fname] => {
                let mut new_p = pwd.clone();
                new_p.push(fname);
                map.insert(new_p, size.parse().unwrap());
            }
            _ => unimplemented!(),
        });
    dbg!(&map);
}
